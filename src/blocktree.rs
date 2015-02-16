use std::collections::hash_map::{self, HashMap};

use block::{HashedBlockExt, SignedBlockExt};
use crypto::{gen_keypair, HashDigest, PublicKey};
use error::{SimplesError, SimplesResult};
use simples_pb::{BlockPatch, HashedBlock, Transaction};
use store::{ProtoStore, KeyValueStore};
use tx::{TransactionBuilder, TransactionExt};

const GENESIS_FIELD: &'static str = "meta:genesis";
const HEAD_FIELD: &'static str = "meta:head";

fn create_genesis_block(tx: Transaction) -> SimplesResult<HashedBlock> {
    if tx.get_commit().get_bounty() != 0 || tx.get_commit().has_bounty_pk() {
        return Err(SimplesError::new(
            "Transactions must not have a bounty set in a genesis block."));
    }
    try!(tx.check_signatures());
    let mut genesis = HashedBlock::new();
    genesis.mut_signed_block().mut_block().mut_transactions().push(tx);
    genesis.compute_hash();
    Ok(genesis)
}

pub struct GenesisBuilder {
    transfers: Vec<(PublicKey, u64)>
}

impl GenesisBuilder {
    pub fn new() -> GenesisBuilder {
        GenesisBuilder {
            transfers: vec![]
        }
    }

    pub fn add_transfer(&mut self, destination: PublicKey, tokens: u64) {
        self.transfers.push((destination, tokens));
    }

    pub fn build(self) -> HashedBlock {
        let (public_key, secret_key) = gen_keypair();
        let mut tx_builder = TransactionBuilder::new();
        let mut op_num = 0u32;
        for (destination, tokens) in self.transfers.into_iter() {
            tx_builder.add_transfer(
                &secret_key, &public_key, &destination, tokens, op_num);
            op_num += 1;
        }
        let genesis_tx = tx_builder.build().unwrap();
        assert!(genesis_tx.check_signatures().is_ok());
        create_genesis_block(genesis_tx).unwrap()
    }
}

// fn block_to_patch(block: &HashedBlock) -> BlockPatch {


// }

fn format_block_key(block_hash: &HashDigest) -> String {
    format!("block:{}", block_hash)
}

fn format_patch_key(block_hash: &HashDigest) -> String {
    format!("patch:{}", block_hash)
}

pub struct BlockTreeStore<Store: KeyValueStore> {
    proto_store: ProtoStore<Store>,
}

impl<Store: KeyValueStore> BlockTreeStore<Store> {
    pub fn new(kv_store: Store, new_genesis: Option<&HashedBlock>)
               -> SimplesResult<Self>
    {
        let mut proto_store = ProtoStore::new(kv_store);
        match (try!(proto_store.kv_store.get_bytes(GENESIS_FIELD.as_bytes())),
               new_genesis) {
            (Some(genesis_hash_raw), Some(new_genesis_block)) => {
                let genesis_hash =
                    try!(HashDigest::from_bytes(&genesis_hash_raw[]));
                let new_genesis_hash = try!(new_genesis_block.get_hash_digest());
                if new_genesis_hash != genesis_hash {
                    return Err(SimplesError::new(&format!(
                        "Blocktree already has genesis {} != {} (requested)",
                        genesis_hash, new_genesis_hash)[]));
                }
                println!("Blocktree already has genesis {}.", genesis_hash);
            },
            (Some(genesis_hash_raw), None) => {
                let genesis_hash =
                    try!(HashDigest::from_bytes(&genesis_hash_raw[]));
                println!("Blocktree has genesis {}.", genesis_hash);
            },
            (None, Some(new_genesis_block)) => {
                let new_genesis_hash = try!(new_genesis_block.get_hash_digest());
                println!("Store doesn't have a genesis block. Setting to {}.",
                         new_genesis_hash);
                try!(proto_store.kv_store.set_bytes(
                    GENESIS_FIELD.as_bytes(), &new_genesis_hash.0[]));
                try!(proto_store.kv_store.set_bytes(
                    HEAD_FIELD.as_bytes(), &new_genesis_hash.0[]));
                try!(proto_store.set_message(&new_genesis_hash.0[],
                                             new_genesis_block));
            },
            (None, None) => return Err(SimplesError::new(
                "No genesis block was specified and store doesn't contain one."))
        }
        Ok(BlockTreeStore { proto_store: proto_store })
    }

    pub fn get_head(&self) -> SimplesResult<HashedBlock> {
        let head_hash = try!(self.get_head_hash());
        let maybe_head: Option<HashedBlock> =
            try!(self.proto_store.get_message(&head_hash.0[]));
        assert!(maybe_head.is_some(), "FATAL: Corrupted blocktree store,
head block missing from kv-store ");
        Ok(maybe_head.unwrap())
    }

    pub fn get_head_hash(&self) -> SimplesResult<HashDigest> {
        let maybe_head_hash_raw =
            try!(self.proto_store.kv_store.get_bytes(HEAD_FIELD.as_bytes()));
        assert!(maybe_head_hash_raw.is_some(),
                "FATAL: Corrupted blocktree, store is headless.");

        let maybe_head_hash = HashDigest::from_bytes(
            &maybe_head_hash_raw.unwrap()[]);
        assert!(maybe_head_hash.is_ok(),
                "FATAL: Corrupted blocktree, head block has invalid hash.");

        Ok(maybe_head_hash.unwrap())
    }

    pub fn set_head(&mut self, new_head: HashedBlock) -> SimplesResult<()> {
        try!(new_head.verify());
        try!(self.set_block(&new_head));
        try!(self.proto_store.kv_store.set_bytes(
            HEAD_FIELD.as_bytes(), new_head.get_hash()));
        Ok(())
    }

    pub fn get_genesis(&self) -> SimplesResult<HashedBlock> {
        let genesis_hash = try!(self.get_genesis_hash());
        let maybe_genesis: Option<HashedBlock> =
            try!(self.proto_store.get_message(&genesis_hash.0[]));
        assert!(maybe_genesis.is_some(), "FATAL: Corrupted blocktree
store, genesis block missing from kv-store ");
        Ok(maybe_genesis.unwrap())
    }

    pub fn get_genesis_hash(&self) -> SimplesResult<HashDigest> {
        let maybe_genesis_hash_raw =
            try!(self.proto_store.kv_store.get_bytes(GENESIS_FIELD.as_bytes()));
        assert!(maybe_genesis_hash_raw.is_some(),
                "FATAL: Corrupted blocktree, genesis hash key: {} is not set.");

        let maybe_genesis_hash = HashDigest::from_bytes(
            &maybe_genesis_hash_raw.unwrap()[]);
        assert!(maybe_genesis_hash.is_ok(),
                "FATAL: Corrupted blocktree, genesis block has invalid hash.");

        Ok(maybe_genesis_hash.unwrap())
    }

    pub fn get_block(&self, block_hash: &HashDigest)
                     -> SimplesResult<Option<HashedBlock>> {
        let db_key = format_block_key(block_hash);
        self.proto_store.get_message(db_key.as_bytes())
    }

    pub fn set_block(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        try!(block.verify());
        let db_key = format_block_key(&try!(block.get_hash_digest()));
        Ok(try!(self.proto_store.set_message(db_key.as_bytes(), block)))
    }
}

#[test]
fn test_create_genesis_empty() {
    let tx = Transaction::new();
    let maybe_genesis = create_genesis_block(tx);
    assert!(maybe_genesis.is_ok());
}

#[test]
fn test_genesis_builder() {
    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let (pk3, sk3) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1, 10);
    builder.add_transfer(pk2, 22);
    builder.add_transfer(pk3, 107);
    let genesis = builder.build();

    assert!(genesis.verify().is_ok());
    let txs = genesis.get_block().get_transactions();
    println!("{:?}", txs);
    assert!(1 == txs.len());
    assert!(3 == txs[0].get_commit().get_transfers().len());
}

#[test]
fn test_blocktree_new_no_genesis() {
    let kv_store1 = HashMap::<Vec<u8>, Vec<u8>>::new();
    let maybe_blocktree1 = BlockTreeStore::new(kv_store1, None);
    assert!(maybe_blocktree1.is_err());
}

#[test]
fn test_blocktree_sets_head_and_genesis() {
    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1, 100);
    builder.add_transfer(pk2, 100);
    let genesis = builder.build();
    assert!(genesis.verify().is_ok());

    let kv_store = HashMap::<Vec<u8>, Vec<u8>>::new();
    let maybe_blocktree = BlockTreeStore::new(kv_store, Some(&genesis));
    assert!(maybe_blocktree.is_ok());

    let blocktree = maybe_blocktree.unwrap();
    let maybe_genesis_hash = blocktree.proto_store.kv_store.get_bytes(
        GENESIS_FIELD.as_bytes()).unwrap();
    assert!(maybe_genesis_hash.is_some());
    assert!(genesis.get_hash() == maybe_genesis_hash.unwrap());
}
