use rustc_serialize::base64::{self, ToBase64};

use crypto::{HashDigest, SecretKey, hash, hash_message,
             sign_message, slice_to_signature};
use error::{SimplesError, SimplesResult};
use simples_pb::{Block, HashedBlock, SignedBlock, Transaction};

pub trait HashedBlockExt {
    fn get_block<'a>(&'a self) -> &'a Block;
    fn compute_hash(&mut self) -> HashDigest;
    fn get_hash_digest(&self) -> SimplesResult<HashDigest>;
    fn verify_hash(&self) -> SimplesResult<()>;
    fn verify(&self) -> SimplesResult<()>;
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

    fn get_hash_digest(&self) -> SimplesResult<HashDigest> {
        HashDigest::from_bytes(self.get_hash())
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

    fn verify(&self) -> SimplesResult<()> {
        try!(self.verify_hash());
        self.get_signed_block().verify_signature()
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

#[test]
fn test_hashed_block_get_block() {
    let mut hashed_block = HashedBlock::new();
    hashed_block.mut_signed_block().mut_block()
        .set_previous(hash(b"test1").0.to_vec());
    assert!(hashed_block.get_signed_block().get_block() == hashed_block.get_block());
}

#[test]
fn test_hashed_block_hash_integrity() {
    let mut hashed_block = HashedBlock::new();
    hashed_block.mut_signed_block().mut_block()
        .set_previous(hash(b"test123").0.to_vec());
    assert!(hashed_block.verify_hash().is_err());
    hashed_block.compute_hash();
    assert!(hashed_block.verify_hash().is_ok());

    hashed_block.mut_signed_block().mut_block()
        .set_previous(hash(b"test123.").0.to_vec());
    assert!(hashed_block.verify_hash().is_err());
    hashed_block.compute_hash();
    assert!(hashed_block.verify_hash().is_ok());
}

// #[test]
// fn test_hashed_block_sign_integrity() {
//     let mut hashed_block = HashedBlock::new();
//     hashed_block.mut_signed_block().mut_mut_block().set_hash(hash("test1"));

//     assert!(hashed_block.verify_hash().is_err());
//     hashed_block.compute_hash();
//     assert!(hashed_block.verify_hash().is_ok());
//     println!("Block hash: {}",
//             &hashed_block.get_hash()[].to_base64(base64::STANDARD));
// }
