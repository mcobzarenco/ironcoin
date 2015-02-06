use rustc_serialize::base64::{self, ToBase64};

use crypto::{HashDigest, SecretKey, hash_message,
             sign_message, slice_to_signature};
use simples_pb::{HashedBlock, SignedBlock, Block};
use store::{ProtoStore, KeyValueStore};
use error::{SimplesResult};

pub trait HashedBlockExt {
    fn get_block<'a>(&'a self) -> &'a Block;
    fn compute_hash(&mut self);
    fn valid_hash(&self) -> bool;
}

impl HashedBlockExt for HashedBlock {
    fn get_block<'a>(&'a self) -> &'a Block {
        self.get_signed_block().get_block()
    }

    fn compute_hash(&mut self) {
        let HashDigest(hash_bytes) = hash_message(self.get_signed_block());
        self.set_hash(hash_bytes.to_vec());
    }

    fn valid_hash(&self) -> bool {
        &hash_message(self.get_signed_block()).0[] == &self.get_hash()[]
    }
}

pub trait SignedBlockExt {
    fn sign(&mut self, secret_key: &SecretKey);
    fn valid_sign(&self) -> bool;
}

impl SignedBlockExt for SignedBlock {
    fn sign(&mut self, secret_key: &SecretKey) {
        let signature = sign_message(secret_key, self.get_block());
        self.set_signature(signature.0.to_vec());
    }

    fn valid_sign(&self) -> bool {
        true
    }
}

const GENESIS_FIELD: &'static [u8] = b"genesis";
const CHAIN_HEAD_FIELD: &'static [u8] = b"blockchain-head";

fn valid_block(block: &HashedBlock) -> bool {
    block.valid_hash() && block.get_signed_block().valid_sign()
}

pub struct BlockStore<Store: KeyValueStore> {
    proto_store: ProtoStore<Store>
}

impl<Store: KeyValueStore> BlockStore<Store> {
    pub fn new(kv_store: Store) -> Self {
        BlockStore { proto_store: ProtoStore::new(kv_store) }
    }

    pub fn get_head(&self) -> SimplesResult<Option<HashedBlock>> {
        self.proto_store.get(CHAIN_HEAD_FIELD)
    }

    pub fn set_head(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        assert!(valid_block(block));
        Ok(try!(self.proto_store.set(CHAIN_HEAD_FIELD, block)))
    }

    pub fn get_genesis(&self) -> SimplesResult<Option<HashedBlock>> {
        self.proto_store.get(GENESIS_FIELD)
    }

    pub fn set_genesis(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        assert!(valid_block(block));
        Ok(try!(self.proto_store.set(GENESIS_FIELD, block)))
    }

    pub fn get_block(&self, hash: &HashDigest)
                     -> SimplesResult<Option<HashedBlock>> {
        let maybe_block = self.proto_store.get(&hash.0[]);
        match maybe_block {
            Ok(Some(ref block)) => { assert!(valid_block(block)); },
            _ => ()
        };
        maybe_block
    }

    pub fn set_block(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        assert!(valid_block(block));
        Ok(try!(self.proto_store.set(&block.get_hash()[], block)))
    }
}

#[test]
fn test_hashed_block_ext() {
    let mut hashed_block = HashedBlock::new();
    assert!(false == hashed_block.valid_hash());
    hashed_block.compute_hash();
    assert!(true == hashed_block.valid_hash());
    println!("Block hash: {}",
            &hashed_block.get_hash()[].to_base64(base64::STANDARD));
}

#[test]
fn test_signed_block_ext() {
}
