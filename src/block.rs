use rustc_serialize::base64::{self, ToBase64};

use crypto::{HashDigest, SecretKey, hash_message,
             sign_message, slice_to_signature};
use simples_pb::{HashedBlock, SignedBlock, Block};
use store::{ProtoStore, KeyValueStore};
use error::{SimplesError, SimplesResult};

pub trait HashedBlockExt {
    fn get_block<'a>(&'a self) -> &'a Block;
    fn compute_hash(&mut self) -> HashDigest;
    fn verify_hash(&self) -> SimplesResult<()>;
}

impl HashedBlockExt for HashedBlock {
    fn get_block<'a>(&'a self) -> &'a Block {
        self.get_signed_block().get_block()
    }

    fn compute_hash(&mut self) -> HashDigest {
        let hash_digest = hash_message(self.get_signed_block());
        self.set_hash(hash_digest.0.to_vec());
        hash_digest
    }

    fn verify_hash(&self) -> SimplesResult<()> {
        let computed_hash = hash_message(self.get_signed_block());
        let block_hash = try!(HashDigest::from_bytes(&self.get_hash()[]));
        if computed_hash == block_hash { Ok(()) }
        else { Err(SimplesError::new(&format!(
            "Block has invalid hash: {} != {} (actual)",
            block_hash, computed_hash)[]))
        }
    }
}

pub trait SignedBlockExt {
    fn sign(&mut self, secret_key: &SecretKey);
    fn verify_signature(&self) -> SimplesResult<()>;
}

impl SignedBlockExt for SignedBlock {
    fn sign(&mut self, secret_key: &SecretKey) {
        let signature = sign_message(secret_key, self.get_block());
        self.set_signature(signature.0.to_vec());
    }

    fn verify_signature(&self) -> SimplesResult<()> {
        Ok(())
    }
}

const GENESIS_FIELD: &'static str = "genesis";
const CHAIN_HEAD_FIELD: &'static str = "blockchain-head";

fn verify_block(block: &HashedBlock) -> SimplesResult<()> {
    try!(block.verify_hash());
    block.get_signed_block().verify_signature()
}

pub struct BlockStore<Store: KeyValueStore> {
    proto_store: ProtoStore<Store>,
    head_block: HashedBlock,
    genesis_block: HashedBlock
}

impl<Store: KeyValueStore> BlockStore<Store> {
    pub fn new_with_genesis(kv_store: Store, genesis: &HashedBlock)
                            -> SimplesResult<Self>
    {
        let mut proto_store = ProtoStore::new(kv_store);
        let new_genesis_hash =
            try!(HashDigest::from_bytes(genesis.get_hash()));
        let maybe_genesis_block  =
            try!(proto_store.get(GENESIS_FIELD.as_bytes()));
        if maybe_genesis_block.is_none()  {
            try!(proto_store.set(GENESIS_FIELD.as_bytes(), genesis));
            try!(proto_store.set(CHAIN_HEAD_FIELD.as_bytes(), genesis));
        } else {
            let genesis_block: HashedBlock = maybe_genesis_block.unwrap();
            let maybe_genesis_hash =
                HashDigest::from_bytes(genesis_block.get_hash());
            if maybe_genesis_hash.is_err() ||
                maybe_genesis_hash.unwrap() != new_genesis_hash {
                    try!(proto_store.set(GENESIS_FIELD.as_bytes(), genesis));
                    try!(proto_store.set(CHAIN_HEAD_FIELD.as_bytes(), genesis));
            }
        }
        Ok(BlockStore {
            proto_store: proto_store,
            head_block: genesis.clone(),
            genesis_block: genesis.clone()
        })
    }

    pub fn new_from_existing(kv_store: Store) -> SimplesResult<Self> {
        let proto_store = ProtoStore::new(kv_store);
        let maybe_genesis_block =
            try!(proto_store.get(GENESIS_FIELD.as_bytes()));
        if maybe_genesis_block.is_none() {
            return Err(SimplesError::new(&format!(
                "No genesis block in DB at key={}", GENESIS_FIELD)));
        }
        let genesis_block = maybe_genesis_block.unwrap();
        println!("Got genesis block");

        let maybe_head_block =
            try!(proto_store.get(CHAIN_HEAD_FIELD.as_bytes()));
        if maybe_head_block.is_none() {
            return Err(SimplesError::new(&format!(
                "No head block in DB at key={}", CHAIN_HEAD_FIELD)));
        }
        let head_block = maybe_head_block.unwrap();
        println!("Got head block");
        Ok(BlockStore {
            proto_store: proto_store,
            head_block: head_block,
            genesis_block: genesis_block
        })
    }

    pub fn get_head(&self) -> &HashedBlock { &self.head_block }

    pub fn get_head_hash(&self) -> HashDigest {
        let maybe_hash = HashDigest::from_bytes(self.head_block.get_hash());
        assert!(maybe_hash.is_ok(),
                "FATAL: Corrupted DB, head block has invalid hash.");
        maybe_hash.unwrap()
    }

    pub fn set_head(&mut self, new_head: HashedBlock) -> SimplesResult<()> {
        try!(verify_block(&new_head));
        try!(self.set_block(&new_head));
        try!(self.proto_store.set(CHAIN_HEAD_FIELD.as_bytes(), &new_head));
        self.head_block = new_head;
        Ok(())
    }

    pub fn get_genesis(&self) -> &HashedBlock { &self.genesis_block }

    pub fn get_genesis_hash(&self) -> HashDigest {
        let maybe_hash = HashDigest::from_bytes(self.genesis_block.get_hash());
        assert!(maybe_hash.is_ok(),
                "FATAL: Corrupted DB, genesis block has invalid hash.");
        maybe_hash.unwrap()
    }

    pub fn get_block(&self, hash: &HashDigest)
                     -> SimplesResult<Option<HashedBlock>> {
        let maybe_block = self.proto_store.get(&hash.0[]);
        match maybe_block {
            Ok(Some(ref block)) => { try!(verify_block(block)); },
            _ => ()
        };
        maybe_block
    }

    pub fn set_block(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        try!(verify_block(block));
        Ok(try!(self.proto_store.set(&block.get_hash()[], block)))
    }
}

#[test]
fn test_hashed_block_ext() {
    let mut hashed_block = HashedBlock::new();
    assert!(hashed_block.verify_hash().is_err());
    hashed_block.compute_hash();
    assert!(hashed_block.verify_hash().is_ok());
    println!("Block hash: {}",
            &hashed_block.get_hash()[].to_base64(base64::STANDARD));
}

#[test]
fn test_signed_block_ext() {
}
