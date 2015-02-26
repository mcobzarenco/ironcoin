use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::slice::bytes::copy_memory;

use protobuf::MessageStatic;
use rustc_serialize::base64::{self, ToBase64};
use sodiumoxide::crypto::hash::sha512::{self, HASHBYTES};
use sodiumoxide::crypto::sign::ed25519::{
    self, PUBLICKEYBYTES, SECRETKEYBYTES, SIGNATUREBYTES};

use error::{IroncError, IroncResult};

// HashDigest:

pub struct HashDigest(pub [u8; HASHBYTES]);

impl HashDigest {
    pub fn as_slice(&self) -> &[u8] { &self.0 }

    pub fn from_slice(bytes: &[u8]) -> IroncResult<HashDigest> {
        if bytes.len() != HASHBYTES {
            Err(IroncError::new(&format!(
                "Invalid length for a hash {} != {} (required).",
                bytes.len(), HASHBYTES)))
        } else {
            let mut digest = HashDigest([0; HASHBYTES]);
            copy_memory(&mut digest.0, bytes);
            Ok(digest)
        }
    }

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

    pub fn multiply_u8_in_place(&mut self, other: u8) {
        let mut quot = 0u16;
        for index in range(0, HASHBYTES) {
            quot = self.0[index] as u16 * other as u16 + quot;
            self.0[index] = (quot & 0x00ffu16) as u8;
            quot = quot >> 8;
        }
    }

    pub fn add_in_place(&mut self, other: &HashDigest) {
        let mut rem = 0u16;
        for index in range(0, HASHBYTES) {
            rem = self.0[index] as u16  + other.0[index] as u16 + rem;
            self.0[index] = (rem & 0x00ffu16) as u8;
            rem = rem >> 8;
        }
    }
}

impl Clone for HashDigest {
    fn clone(&self) -> Self { HashDigest::from_slice(&self.0).unwrap() }
}

impl PartialEq for HashDigest {
    fn eq(&self, other: &HashDigest) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for HashDigest {}

impl Hash for HashDigest {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.as_slice()); }
}

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
        write!(formatter, "{:?}", self.as_slice())
    }
}

impl fmt::Display for HashDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_slice().to_base64(base64::STANDARD))
    }
}

pub fn hash(bytes: &[u8]) -> HashDigest {
    HashDigest(sha512::hash(bytes).0)
}

// PublicKey:

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PublicKey(pub [u8; PUBLICKEYBYTES]);

impl PublicKey {
    pub fn as_slice(&self) -> &[u8] { &self.0 }

    pub fn from_slice(bytes: &[u8]) -> IroncResult<PublicKey> {
        if bytes.len() != PUBLICKEYBYTES {
            Err(IroncError::new(
                &format!("Invalid public key length {} != {} (required)",
                         bytes.len(), PUBLICKEYBYTES)))
        } else {
            let mut public_key = PublicKey([0; PUBLICKEYBYTES]);
            copy_memory(&mut public_key.0, bytes);
            Ok(public_key)
        }
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_slice().to_base64(base64::STANDARD))
    }
}

// SecretKey:

pub struct SecretKey(pub [u8; SECRETKEYBYTES]);

impl SecretKey {
    pub fn as_slice(&self) -> &[u8] { &self.0 }

    pub fn from_slice(bytes: &[u8]) -> IroncResult<SecretKey> {
        if bytes.len() != SECRETKEYBYTES {
            Err(IroncError::new(
                &format!("Invalid secret key length {} != {} (required)",
                         bytes.len(), SECRETKEYBYTES)))
        } else {
            let mut secret_key = SecretKey([0; SECRETKEYBYTES]);
            copy_memory(&mut secret_key.0, bytes);
            Ok(secret_key)
        }
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) { copy_memory(&mut self.0, &[0; SECRETKEYBYTES]); }
}

impl Clone for SecretKey {
    fn clone(&self) -> Self { SecretKey::from_slice(&self.0).unwrap() }
}

impl PartialEq for SecretKey {
    fn eq(&self, other: &SecretKey) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for SecretKey {}

impl Hash for SecretKey {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.as_slice()); }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.as_slice())
    }
}

impl fmt::Display for SecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_slice().to_base64(base64::STANDARD))
    }
}

pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = ed25519::gen_keypair();
    (PublicKey(pk.0), SecretKey(sk.0))
}

// Signature:

pub struct Signature(pub [u8; SIGNATUREBYTES]);

impl Signature {
    pub fn as_slice(&self) -> &[u8] { &self.0 }

    pub fn from_slice(bytes: &[u8]) -> IroncResult<Signature> {
        if bytes.len() != SIGNATUREBYTES {
            Err(IroncError::new(
                &format!("Invalid signature length {} != {} (required)",
                         bytes.len(), SIGNATUREBYTES)))
        } else {
            let mut signature = Signature([0; SIGNATUREBYTES]);
            copy_memory(&mut signature.0, bytes);
            Ok(signature)
        }
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self { Signature::from_slice(&self.0).unwrap() }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Signature) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Signature {}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.as_slice()); }
}

impl fmt::Debug for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.as_slice())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_slice().to_base64(base64::STANDARD))
    }
}

pub fn sign(secret_key: &SecretKey, message: &[u8]) -> Signature {
    Signature(ed25519::sign_detached(message, &ed25519::SecretKey(secret_key.0)).0)
}

pub fn verify_signature(public_key: &PublicKey, message: &[u8],
                        signature: &Signature) -> IroncResult<()> {
    if ed25519::verify_detached(&ed25519::Signature(signature.0),
                                message, &ed25519::PublicKey(public_key.0)) {
        Ok(())
    } else { Err(IroncError::new("Invalid signature.")) }
}

// Utilities for crypto on protobufs:

pub fn hash_message<M: MessageStatic>(message: &M) -> HashDigest {
    hash(&message.write_to_bytes().unwrap())
}

pub fn sign_message<M: MessageStatic>(
    secret_key: &SecretKey, message: &M) -> Signature {
    let msg_bytes = &message.write_to_bytes().unwrap();
    sign(secret_key, msg_bytes)
}

pub fn verify_signed_message<M: MessageStatic>(
    public_key: &PublicKey, message: &M, signature: &Signature)
    -> IroncResult<()> {
    let msg_bytes: &[u8] = &message.write_to_bytes().unwrap();
    verify_signature(public_key, msg_bytes, signature)
}

/*****  Tests  *****/

use rustc_serialize::json;

#[test]
fn test_digest_from_u64() {
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
fn test_digest_multiply_u8() {
    let mut hash1 = HashDigest::from_u64(101);
    let mut hash2 = HashDigest::from_u64(256);
    hash1.multiply_u8_in_place(7);
    hash2.multiply_u8_in_place(2);
    assert!(HashDigest::from_u64(707) == hash1);
    assert!(HashDigest::from_u64(512) == hash2);
    hash1.multiply_u8_in_place(7);
    hash2.multiply_u8_in_place(4);
    assert!(HashDigest::from_u64(4949) == hash1);
    assert!(HashDigest::from_u64(2048) == hash2);
}

#[test]
fn test_digest_add() {
    let mut hash1 = HashDigest::from_u64(101);
    let mut hash2 = HashDigest::from_u64(256);
    hash1.add_in_place(&HashDigest::from_u64(100001));
    hash2.add_in_place(&HashDigest::from_u64(256));
    assert!(HashDigest::from_u64(100102) == hash1);
    assert!(HashDigest::from_u64(512) == hash2);
    hash1.add_in_place(&HashDigest::from_u64(256));
    hash2.add_in_place(&HashDigest::from_u64(1000));
    assert!(HashDigest::from_u64(100358) == hash1);
    assert!(HashDigest::from_u64(1512) == hash2);
}

// #[test]
// fn test_digest_multiply() {
//     let mut hash1 = HashDigest::from_u64(256);
//     let mut hash2 = HashDigest::from_u64(256);
//     hash1.multiply_in_place(&hash2);
//     assert!(hash1.0[0] == 0);
//     assert!(hash1.0[1] == 1);
//     for i in range(1, HASHBYTES) {
//         assert!(hash1.0[i] == 0);
//     }

//     let mut hash3 = HashDigest::from_u64(65536);
//     println!("* hash3: {:?}", hash3);
//     println!("* hash3 == hash3: {}", hash3 == hash3);
//     println!("* hash3 > hash3: {}", hash3 > hash3);
//     println!("* hash3 > hash2: {}", hash3 > hash2);
//     println!("* hash3 < hash2: {}", hash3 < hash2);
//     // hash3.multiply
// }

#[test]
fn test_digest_ord() {
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

// #[test]
// fn test_digest_encoding() {
//     let hash1 = hash(b"hello world2");
//     let hash2 = HashDigest::from_u64(123456);

//     let hash1_enc = json::encode(&hash1).unwrap();
//     let hash2_enc = json::encode(&hash2).unwrap();

//     let hash1_dec: HashDigest = json::decode(&hash1_enc).unwrap();
//     let hash2_dec: HashDigest = json::decode(&hash2_enc).unwrap();

//     println!("hash1 = {}", hash1_enc);
//     println!("hash2 = {}", hash2_enc);
//     println!("hash1_dec = {}", hash1_dec);
//     println!("hash2_dec = {}", hash2_dec);

//     assert!(hash1 == hash1_dec);
//     assert!(hash2 == hash2_dec);
// }

// #[test]
// fn test_digest() {
//     let HashDigest(target_hash_bytes) = hash(b"hello world2");
//     let target_hash = HashDigest(target_hash_bytes);
//     let now_secs = now_utc().to_timespec().sec;
//     println!("now_secs = {}", now_secs);
//     println!("target_hash = {:?}", target_hash);
//     for i in range(0, 100) {
//         let HashDigest(hash_bytes) = hash(
//             format!("{}", now_secs + i).as_bytes());
//         let hash = HashDigest(hash_bytes);
//         if hash > target_hash {
//             println!("hash({}) = {:?}", now_secs + i, hash);
//         }
//     }
// }
