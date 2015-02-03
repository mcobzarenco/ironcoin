use simples_pb::{HashedBlock, SignedBlock, Block};

use protobuf::MessageStatic;
use sodiumoxide::crypto::hash::sha512::{self, Digest, HASHBYTES};
use sodiumoxide::crypto::sign::ed25519::{
    self, PublicKey, SecretKey, PUBLICKEYBYTES, SECRETKEYBYTES,
    Signature, SIGNATUREBYTES, sign_detached, verify_detached};
use rustc_serialize::base64::{self, ToBase64};

use tx::{slice_to_signature};

fn hash_message<M: MessageStatic>(message: &M) -> Digest {
    let msg_bytes = &message.write_to_bytes().unwrap()[];
    sha512::hash(msg_bytes)
}

fn sign_message<M: MessageStatic>(
    secret_key: &SecretKey, message: &M) -> Signature
{
    let msg_bytes = &message.write_to_bytes().unwrap()[];
    sign_detached(msg_bytes, secret_key)
}

fn verify_signed_message<M: MessageStatic>(
    public_key: &PublicKey, message: &M, signature: &Signature) -> bool
{
    let msg_bytes = &message.write_to_bytes().unwrap()[];
    verify_detached(signature, msg_bytes, public_key)
}

pub trait HashedBlockExt {
    fn compute_hash(&mut self);
    fn valid_hash(&self) -> bool;
}

impl HashedBlockExt for HashedBlock {
    fn compute_hash(&mut self) {
        let Digest(hash_bytes) = hash_message(self.get_signed_block());
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
