extern crate sodiumoxide;

use std::fmt;
use std::cmp::{Ordering};
use std::slice::bytes::copy_memory;

use protobuf::MessageStatic;
use sodiumoxide::crypto::hash::sha512;
use sodiumoxide::crypto::sign::ed25519::{
    PUBLICKEYBYTES, SECRETKEYBYTES, SIGNATUREBYTES,
    sign_detached, verify_detached};
use time::now_utc;

pub use sodiumoxide::crypto::hash::sha512::HASHBYTES;
pub use sodiumoxide::crypto::sign::ed25519::{PublicKey, SecretKey, Signature};

pub struct HashDigest(pub [u8; HASHBYTES]);

pub fn hash(bytes: &[u8]) -> HashDigest {
    HashDigest(sha512::hash(bytes).0)
}

impl HashDigest {
    pub fn from_u64(mut value: u64) -> HashDigest {
        let mut proof_hash = HashDigest([0; HASHBYTES]);
        let mut index = 0us;
        while value > 0u64 {
            proof_hash.0[index] = (value & 0xffu64) as u8;
            value = value >> 8;
            index += 1;
        }
        proof_hash
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<HashDigest> {
        match bytes.len() != HASHBYTES {
            true => None,
            false => {
                let mut digest = HashDigest([0; HASHBYTES]);
                copy_memory(&mut digest.0[], bytes);
                Some(digest)
            }
        }
    }

    pub fn multiply_in_place(&mut self, other: &HashDigest) {
        let mut quot = 0u16;
        for index in range(0, HASHBYTES) {
            quot = self.0[index] as u16 * other.0[index] as u16 + quot;
            self.0[index] = (quot & 0x00ffu16) as u8;
            quot = quot >> 8;
        }
    }
}

impl PartialEq for HashDigest {
    fn eq(&self, other: &HashDigest) -> bool {
        for (byte1, byte2) in self.0.iter().zip(other.0.iter()) {
            if byte1 != byte2 {
                return false;
            }
        }
        true
    }
}

impl Eq for HashDigest {}

impl PartialOrd for HashDigest {
    fn partial_cmp(&self, other: &HashDigest) -> Option<Ordering> {
        for (byte1, byte2) in self.0.iter().rev().zip(
            other.0.iter().rev()) {
            if byte1 > byte2 {
                return Some(Ordering::Greater);
            } else if byte2 > byte1 {
                return Some(Ordering::Less);
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Ord for HashDigest {
    fn cmp(&self, other: &HashDigest) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for HashDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", &self.0[])
    }
}

pub fn sign(secret_key: &SecretKey, message: &[u8]) -> Signature {
    sign_detached(message, secret_key)
}

pub fn verify_signature(
    public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    verify_detached(signature, message, public_key)
}

// Utilities for crypto on protobufs:

pub fn hash_message<M: MessageStatic>(message: &M) -> HashDigest {
    hash(&message.write_to_bytes().unwrap()[])
}

pub fn sign_message<M: MessageStatic>(
    secret_key: &SecretKey, message: &M) -> Signature
{
    let msg_bytes = &message.write_to_bytes().unwrap()[];
    sign(secret_key, msg_bytes)
}

pub fn verify_signed_message<M: MessageStatic>(
    public_key: &PublicKey, message: &M, signature: &Signature) -> bool
{
    let msg_bytes: &[u8] = &message.write_to_bytes().unwrap()[];
    verify_signature(public_key, msg_bytes, signature)
}

pub fn slice_to_pk(bytes: &[u8]) -> Option<PublicKey> {
    if bytes.len() != PUBLICKEYBYTES { return None; }
    let mut key:[u8; PUBLICKEYBYTES] = [0; PUBLICKEYBYTES];
    for i in range(0, PUBLICKEYBYTES) {
        key[i] = bytes[i];
    }
    Some(PublicKey(key))
}

pub fn slice_to_sk(bytes: &[u8]) -> Option<SecretKey> {
    if bytes.len() != SECRETKEYBYTES { return None; }
    let mut key:[u8; SECRETKEYBYTES] = [0; SECRETKEYBYTES];
    for i in range(0, SECRETKEYBYTES) {
        key[i] = bytes[i];
    }
    Some(SecretKey(key))
}

pub fn slice_to_signature(bytes: &[u8]) -> Option<Signature> {
    if bytes.len() != SIGNATUREBYTES { return None; }
    let mut sign:[u8; SIGNATUREBYTES] = [0; SIGNATUREBYTES];
    for i in range(0, SIGNATUREBYTES) {
        sign[i] = bytes[i];
    }
    Some(Signature(sign))
}

// Tests:

#[test]
fn test_from_u64() {
    let mut hash1 = HashDigest::from_u64(2100);
    let mut hash2 = HashDigest::from_u64(65535);

    assert!(hash1.0[0] == 52);
    assert!(hash1.0[1] == 8);
    assert!(hash2.0[0] == 255);
    assert!(hash2.0[1] == 255);
    for i in range(3, HASHBYTES) {
        assert!(hash1.0[i] == 0);
        assert!(hash2.0[i] == 0);
    }
}

#[test]
fn test_hash_multiply() {
    let mut hash1 = HashDigest::from_u64(2100);
    let hash2 = HashDigest::from_u64(65535);
    hash1.multiply_in_place(&hash2);
    assert!(hash1.0[0] == 204);
    assert!(hash1.0[1] == 43);
    assert!(hash1.0[2] == 8);
    for i in range(3, HASHBYTES) {
        assert!(hash1.0[i] == 0);
    }

    let mut hash3 = HashDigest::from_u64(65536);
    println!("* hash3: {:?}", hash3);
    println!("* hash3 == hash3: {}", hash3 == hash3);
    println!("* hash3 > hash3: {}", hash3 > hash3);
    println!("* hash3 > hash2: {}", hash3 > hash2);
    println!("* hash3 < hash2: {}", hash3 < hash2);
    // hash3.multiply
}

#[test]
fn test_proof_hash_ord() {
    let hashes = [
        HashDigest::from_u64(0), HashDigest::from_u64(2100),
        HashDigest::from_u64(65535), HashDigest::from_u64(65536),
        HashDigest::from_u64(65536 * 100000)];

    for i in range(0, hashes.len() - 1) {
        for j in range(i + 1, hashes.len()) {
            assert!(hashes[i] < hashes[j]);
            assert!(hashes[j] > hashes[i]);
        }
    }
}

#[test]
fn test_proof_hash() {
    let HashDigest(target_hash_bytes) = hash(b"hello world2");
    let target_hash = HashDigest(target_hash_bytes);
    let now_secs = now_utc().to_timespec().sec;
    println!("now_secs = {}", now_secs);
    println!("target_hash = {:?}", target_hash);
    for i in range(0, 100) {
        let HashDigest(hash_bytes) = hash(
            format!("{}", now_secs + i).as_bytes());
        let hash = HashDigest(hash_bytes);
        if hash > target_hash {
            println!("hash({}) = {:?}", now_secs + i, hash);
        }
    }
}
