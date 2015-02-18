use std::collections::hash_map::{self, HashMap};

use protobuf::RepeatedField;

use balance::{BalancePatchExt, LedgerReader, LedgerWriter, LedgerSnapshot,
              Patchable, make_genesis_patch};
use block::{GenesisBuilder, BlockPatchExt, HashedBlockExt, SignedBlockExt};
use crypto::{gen_keypair, HashDigest, PublicKey};
use error::{SimplesError, SimplesResult};
use simples_pb::{Balance, BalancePatch, BlockPatch, HashedBlock, Transaction};
use store::{MessageStore, KeyValueStore, ProtobufStore};
use tx::TransactionExt;

const GENESIS_FIELD: &'static str = "meta:genesis";
const HEAD_FIELD: &'static str = "meta:head";

fn format_balance_key(address: &PublicKey) -> String {
    format!("balance:{}", address)
}

fn format_block_key(block_hash: &HashDigest) -> String {
    format!("block:{}", block_hash)
}

fn format_patch_key(block_hash: &HashDigest) -> String {
    format!("patch:{}", block_hash)
}

pub struct BlockTreeStore<Store: KeyValueStore> {
    store: MessageStore<Store>,
}

impl<Store: KeyValueStore> BlockTreeStore<Store> {
    pub fn new(kv_store: Store, new_genesis: Option<&HashedBlock>)
               -> SimplesResult<Self>
    {
        let mut store = MessageStore::<Store>::new(kv_store);
        let mut blocktree;
        let maybe_genesis_hash =
            match try!(store.get_bytes(GENESIS_FIELD.as_bytes())) {
                Some(hash_raw) => Some(try!(HashDigest::from_bytes(&hash_raw[]))),
                None => None
            };
        match (maybe_genesis_hash, new_genesis) {
            (Some(genesis_hash), Some(new_genesis_block)) => {
                let new_genesis_hash = try!(new_genesis_block.decode_hash());
                if new_genesis_hash != genesis_hash {
                    return Err(SimplesError::new(&format!(
                        "Blocktree already has genesis {} != {} (requested)",
                        genesis_hash, new_genesis_hash)[]));
                }
                println!("Blocktree already has genesis {}.", genesis_hash);
                blocktree = BlockTreeStore { store: store }
            },
            (Some(genesis_hash), None) => {
                println!("Blocktree has genesis {}.", genesis_hash);
                blocktree = BlockTreeStore { store: store }
            },
            (None, Some(new_genesis_block)) => {
                let new_genesis_hash = try!(new_genesis_block.decode_hash());
                println!("Store doesn't have a genesis block. Setting it to {}.",
                         new_genesis_hash);
                let genesis_patch = try!(make_genesis_patch(new_genesis_block));
                let genesis_key = format_block_key(&new_genesis_hash);
                let patch_key = format_patch_key(&new_genesis_hash);
                try!(store.set_bytes(
                    GENESIS_FIELD.as_bytes(), &new_genesis_hash.0[]));
                try!(store.set_bytes(
                    HEAD_FIELD.as_bytes(), &new_genesis_hash.0[]));
                try!(store.set_message(genesis_key.as_bytes(), new_genesis_block));
                try!(store.set_message(patch_key.as_bytes(), &genesis_patch));
                blocktree = BlockTreeStore { store: store };
                println!("genesis_patch={:?}", genesis_patch);
                for patch in genesis_patch.get_patches().iter() {
                    try!(blocktree.apply_patch(patch.clone()));
                }
            },
            (None, None) => return Err(SimplesError::new(
                "No genesis block was specified and store doesn't contain one."))
        }
        Ok(blocktree)
    }

    pub fn get_head(&self) -> SimplesResult<HashedBlock> {
        let head_hash = try!(self.get_head_hash());
        let maybe_head: Option<HashedBlock> =
            try!(self.store.get_message(format_block_key(&head_hash).as_bytes()));
        assert!(maybe_head.is_some(), "FATAL: Corrupted blocktree store,
head block missing from kv-store.");
        Ok(maybe_head.unwrap())
    }

    pub fn get_head_hash(&self) -> SimplesResult<HashDigest> {
        let maybe_head_hash_raw =
            try!(self.store.get_bytes(HEAD_FIELD.as_bytes()));
        assert!(maybe_head_hash_raw.is_some(),
                "FATAL: Corrupted blocktree, store is headless.");

        let maybe_head_hash = HashDigest::from_bytes(
            &maybe_head_hash_raw.unwrap()[]);
        assert!(maybe_head_hash.is_ok(),
                "FATAL: Corrupted blocktree, head block has invalid hash.");

        Ok(maybe_head_hash.unwrap())
    }

    pub fn set_head(&mut self, new_head_hash: &HashDigest) -> SimplesResult<()> {
        let current_head_hash = try!(self.get_head_hash());
        let maybe_new_head = try!(self.get_block(new_head_hash));
        if maybe_new_head.is_none() {
            return Err(SimplesError::new(&format!(
                "Tried to set head to {}, but it doesn't exist in the blocktree",
                new_head_hash)[]))
        }
        let new_head = maybe_new_head.unwrap();
        let new_head_patch = try!(self.get_patch(new_head_hash)).expect(&format!(
            "FATAL: Corrupted blocktree, patch missing for block {}",
            new_head_hash)[]);
        if current_head_hash != try!(new_head.decode_previous()) {
            return Err(SimplesError::new(
                "current_head_hash != new_head.decode_previous() not supported yet"));
        }
        for patch in new_head_patch.get_patches().iter() {
            println!("set_head patch: {:?}", patch);
            try!(self.apply_patch(patch.clone()));
        }
        self.store.set_bytes(HEAD_FIELD.as_bytes(), &new_head_hash.0[])
    }

    pub fn get_genesis(&self) -> SimplesResult<HashedBlock> {
        let genesis_hash = try!(self.get_genesis_hash());
        let genesis_key = format_block_key(&genesis_hash);
        let maybe_genesis: Option<HashedBlock> =
            try!(self.store.get_message(genesis_key.as_bytes()));
        assert!(maybe_genesis.is_some(), "FATAL: Corrupted blocktree
store, genesis block missing from kv-store.");
        Ok(maybe_genesis.unwrap())
    }

    pub fn get_genesis_hash(&self) -> SimplesResult<HashDigest> {
        let maybe_genesis_hash_raw =
            try!(self.store.get_bytes(GENESIS_FIELD.as_bytes()));
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
        self.store.get_message(db_key.as_bytes())
    }

    pub fn insert_block(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        try!(block.verify());
        let block_hash = try!(block.decode_hash());
        let previous_hash = try!(block.decode_previous());
        if previous_hash != try!(self.get_head_hash()) {
            println!("previous_hash != self.get_head_hash() not supported yet");
            return Err(SimplesError::new(
                "previous_hash != self.get_head_hash() not supported yet"));
        }
        let mut block_patch = BlockPatch::new();
        {
            let mut snapshot = LedgerSnapshot::new(self);
            try!(snapshot.apply_block(block));
            block_patch.set_patches(
                RepeatedField::from_vec(snapshot.make_patches()));
        }
        block_patch.encode_previous(&previous_hash);
        let patch_key = format_patch_key(&block_hash);
        let block_key = format_block_key(&block_hash);
        try!(self.store.set_message(patch_key.as_bytes(), &block_patch));
        Ok(try!(self.store.set_message(block_key.as_bytes(), block)))
    }

    pub fn rewind(&self, block_hash: &HashDigest)
                  -> SimplesResult<LedgerSnapshot<Self>>
    {
        let genesis = try!(self.get_genesis_hash());
        let mut snapshot = LedgerSnapshot::new(self);
        let mut current_block = try!(self.get_head_hash());
        while current_block != *block_hash {
            if current_block == genesis {
                return Err(SimplesError::new(
                    "Blocktree cannot rewind head to {} as it's not an ancestor."));
            }
            let maybe_block_patch = try!(self.get_patch(&current_block));
            if maybe_block_patch.is_none() {
                return Err(SimplesError::new(&format!(
                    "FATAL: Corrupted blocktree, patch missing for block {}",
                    current_block)[]));
            }
            let block_patch = maybe_block_patch.unwrap();
            for patch in block_patch.get_patches().iter() {
                try!(snapshot.apply_patch(patch.reverse()));
            }
            current_block = block_patch.decode_previous().unwrap();
        }
        Ok(snapshot)
    }

    pub fn rewind_and_forward(&self, path: &[HashDigest]) ->
        SimplesResult<LedgerSnapshot<Self>>
    {
        if path.len() == 0 {
            return Err(SimplesError::new("rewind_and_forward error: empty path"));
        }
        let snapshot = try!(self.rewind(&path[0]));
        for block_hash in path[1..].iter() {
            let maybe_block_patch = try!(self.get_patch(block_hash));
            if maybe_block_patch.is_none() {
                return Err(SimplesError::new(&format!(
                    "rewind_and_forward error: missing patch for block {}",
                    block_hash)[]));
            }
            let block_patch = maybe_block_patch.unwrap();
        }
        Ok(snapshot)
    }

    pub fn tx_cache(&self) -> LedgerSnapshot<Self> {
        LedgerSnapshot::new(self)
    }

    fn get_patch(&self, block_hash: &HashDigest)
                 -> SimplesResult<Option<BlockPatch>> {
        let db_key = format_patch_key(block_hash);
        self.store.get_message(db_key.as_bytes())
    }

    fn set_patch(&mut self, block_hash: &HashDigest, patch: &BlockPatch)
                 -> SimplesResult<()> {
        let db_key = format_patch_key(block_hash);
        self.store.set_message(db_key.as_bytes(), patch)
    }

    // fn make_block_patch(&self, block: &HashedBlock) -> SimplesResult<BlockPatch> {
    //     let previous_hash =
    //         try!(HashDigest::from_bytes(block.get_block().get_previous()));
    //     if previous_hash != try!(self.get_head_hash()) {
    //         println!("previous_hash != self.get_head_hash() not supported yet");
    //         return Err(SimplesError::new(
    //             "previous_hash != self.get_head_hash() not supported yet"));
    //     }
    //     let mut cache = self.tx_cache();
    //     let txes = block.get_block().get_transactions();
    //     for tx in txes.iter() {
    //         try!(cache.apply_transaction(tx));
    //     }
    //     Ok(cache.make_patches())
    // }
}

impl<Store: KeyValueStore> LedgerReader for BlockTreeStore<Store> {
    fn get_balance(&self, address: &PublicKey) -> SimplesResult<Balance> {
        let db_key = format_balance_key(address);
        match try!(self.store.get_message(db_key.as_bytes())) {
            Some(balance) => Ok(balance),
            None => {
                let mut balance = Balance::new();
                balance.set_tokens(0);
                balance.set_op_index(0);
                Ok(balance)
            }
        }
    }
}

impl<Store: KeyValueStore> LedgerWriter for BlockTreeStore<Store> {
    fn set_balance(&mut self, address: &PublicKey, balance: Balance)
                   -> SimplesResult<()> {
        let db_key = format_balance_key(address);
        self.store.set_message(db_key.as_bytes(), &balance)
    }
}

/*****  Tests  *****/

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

    let maybe_genesis_hash = blocktree.store.get_bytes(
        GENESIS_FIELD.as_bytes()).unwrap();
    let maybe_head_hash = blocktree.store.get_bytes(
        HEAD_FIELD.as_bytes()).unwrap();
    assert!(maybe_genesis_hash.is_some());
    assert!(maybe_head_hash.is_some());

    let genesis_hash =
        HashDigest::from_bytes(&maybe_genesis_hash.unwrap()[]).unwrap();
    let head_hash =
        HashDigest::from_bytes(&maybe_head_hash.unwrap()[]).unwrap();
    assert!(genesis.decode_hash().unwrap() == genesis_hash);
    assert!(genesis_hash == head_hash);
    assert!(blocktree.get_genesis_hash().unwrap() == genesis_hash);
    assert!(blocktree.get_head_hash().unwrap() == genesis_hash);

    let maybe_genesis_block = blocktree.store.get_message(
        format_block_key(&genesis_hash).as_bytes()).unwrap();
    assert!(maybe_genesis_block.is_some());
    assert!(genesis == maybe_genesis_block.unwrap());
    assert!(genesis == blocktree.get_genesis().unwrap());

    let maybe_head_block = blocktree.store.get_message(
        format_block_key(&genesis_hash).as_bytes()).unwrap();
    assert!(maybe_head_block.is_some());
    assert!(genesis == maybe_head_block.unwrap());
    assert!(genesis == blocktree.get_head().unwrap());
}

#[test]
fn test_blocktree_insert_and_rewind() {
    use tx::TransactionBuilder;

    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let (pk3, sk3) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1.clone(), 980);
    builder.add_transfer(pk2.clone(), 73);
    builder.add_transfer(pk3.clone(), 10);
    let genesis = builder.build();
    assert!(genesis.verify().is_ok());

    let kv_store = HashMap::<Vec<u8>, Vec<u8>>::new();
    let mut blocktree = BlockTreeStore::new(kv_store, Some(&genesis)).unwrap();

    let mut tx_builder1 = TransactionBuilder::new();
    tx_builder1.add_transfer(&sk1, &pk1, &pk2, 11, 0);
    tx_builder1.add_transfer(&sk2, &pk2, &pk1, 1, 0);
    let tx1 = tx_builder1.build().unwrap();
    let mut block1 = HashedBlock::new();
    block1.mut_signed_block().mut_block().mut_transactions().push(tx1);
    block1.set_previous_block(&genesis.decode_hash().unwrap());
    let block1_hash = block1.compute_hash();
    blocktree.insert_block(&block1).unwrap();
    assert!(blocktree.set_head(&block1_hash).is_ok());
    assert!(970 == blocktree.get_balance(&pk1).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk1).unwrap().get_op_index());
    assert!(83 == blocktree.get_balance(&pk2).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk2).unwrap().get_op_index());
    assert!(10 == blocktree.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == blocktree.get_balance(&pk3).unwrap().get_op_index());

    let mut tx_builder2 = TransactionBuilder::new();
    tx_builder2.add_transfer(&sk2, &pk2, &pk3, 10, 1);
    let tx2 = tx_builder2.build().unwrap();
    let mut block2 = HashedBlock::new();
    block2.mut_signed_block().mut_block().mut_transactions().push(tx2);
    block2.set_previous_block(&block1_hash);
    let block2_hash = block2.compute_hash();
    blocktree.insert_block(&block2).unwrap();
    assert!(blocktree.set_head(&block2_hash).is_ok());
    assert!(970 == blocktree.get_balance(&pk1).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk1).unwrap().get_op_index());
    assert!(73 == blocktree.get_balance(&pk2).unwrap().get_tokens());
    assert!(2 == blocktree.get_balance(&pk2).unwrap().get_op_index());
    assert!(20 == blocktree.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == blocktree.get_balance(&pk3).unwrap().get_op_index());

    let at_genesis = blocktree.rewind(&genesis.decode_hash().unwrap()).unwrap();
    assert!(980 == at_genesis.get_balance(&pk1).unwrap().get_tokens());
    assert!(0  == at_genesis.get_balance(&pk1).unwrap().get_op_index());
    assert!(73 == at_genesis.get_balance(&pk2).unwrap().get_tokens());
    assert!(0 == at_genesis.get_balance(&pk2).unwrap().get_op_index());
    assert!(10 == at_genesis.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == at_genesis.get_balance(&pk3).unwrap().get_op_index());

    let at_block1 = blocktree.rewind(&block1_hash).unwrap();
    assert!(970 == at_block1.get_balance(&pk1).unwrap().get_tokens());
    assert!(1 == at_block1.get_balance(&pk1).unwrap().get_op_index());
    assert!(83 == at_block1.get_balance(&pk2).unwrap().get_tokens());
    assert!(1 == at_block1.get_balance(&pk2).unwrap().get_op_index());
    assert!(10 == at_block1.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == at_block1.get_balance(&pk3).unwrap().get_op_index());
}
