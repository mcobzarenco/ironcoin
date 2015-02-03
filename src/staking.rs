use std::collections::HashSet;
use std::cmp::{Ordering};
use std::fmt;
use std::num::{UnsignedInt, Int};
use time::now_utc;

use tx::{self, Transaction};
use sodiumoxide::crypto::hash::sha512::{self, Digest, HASHBYTES};

pub struct ProofHash([u8; HASHBYTES]);

impl ProofHash {
    fn from_u64(mut value: u64) -> ProofHash {
        let mut proof_hash = ProofHash([0; HASHBYTES]);
        let mut index = 0us;
        while value > 0u64 {
            proof_hash.0[index] = (value & 0xffu64) as u8;
            value = value >> 8;
            index += 1;
        }
        proof_hash
    }

    fn multiply_in_place(&mut self, other: &ProofHash) {
        let mut quot = 0u16;
        for index in range(0, HASHBYTES) {
            quot = self.0[index] as u16 * other.0[index] as u16 + quot;
            self.0[index] = (quot & 0x00ffu16) as u8;
            quot = quot >> 8;
        }
    }
}

impl PartialEq for ProofHash {
    fn eq(&self, other: &ProofHash) -> bool {
        for (byte1, byte2) in self.0.iter().zip(other.0.iter()) {
            if byte1 != byte2 {
                return false;
            }
        }
        true
    }
}

impl Eq for ProofHash {}

impl PartialOrd for ProofHash {
    fn partial_cmp(&self, other: &ProofHash) -> Option<Ordering> {
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

impl Ord for ProofHash {
    fn cmp(&self, other: &ProofHash) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for ProofHash {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", &self.0[])
    }
}

pub struct ThreadedStaker {
    tx_pool: HashSet<String>
}

#[test]
fn test_from_u64() {
    let mut hash1 = ProofHash::from_u64(2100);
    let mut hash2 = ProofHash::from_u64(65535);

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
    let mut hash1 = ProofHash::from_u64(2100);
    let hash2 = ProofHash::from_u64(65535);
    hash1.multiply_in_place(&hash2);
    assert!(hash1.0[0] == 204);
    assert!(hash1.0[1] == 43);
    assert!(hash1.0[2] == 8);
    for i in range(3, HASHBYTES) {
        assert!(hash1.0[i] == 0);
    }

    let mut hash3 = ProofHash::from_u64(65536);
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
        ProofHash::from_u64(0), ProofHash::from_u64(2100),
        ProofHash::from_u64(65535), ProofHash::from_u64(65536),
        ProofHash::from_u64(65536 * 100000)];

    for i in range(0, hashes.len() - 1) {
        for j in range(i + 1, hashes.len()) {
            assert!(hashes[i] < hashes[j]);
            assert!(hashes[j] > hashes[i]);
        }
    }
}

#[test]
fn test_proof_hash() {
    let Digest(target_hash_bytes) = sha512::hash(b"hello world2");
    let target_hash = ProofHash(target_hash_bytes);
    let now_secs = now_utc().to_timespec().sec;
    println!("now_secs = {}", now_secs);
    println!("target_hash = {:?}", target_hash);
    for i in range(0, 100) {
        let Digest(hash_bytes) = sha512::hash(
            format!("{}", now_secs + i).as_bytes());
        let hash = ProofHash(hash_bytes);
        if hash > target_hash {
            println!("hash({}) = {:?}", now_secs + i, hash);
        }
    }
}

// use std::sync::mpsc::{channel, Sender, Receiver};
// struct SimplesService {
//     tx_sender: Sender<simples_pb::Transaction>,
// }

// impl Service for SimplesService {
//     fn pub_transaction(
//         &mut self, request: simples_pb::PublishTransactionRequest) ->
//         SimplesResult<simples_pb::PublishTransactionResponse> {
//             println!("{:?}", request.get_transaction());
//             let response = simples_pb::PublishTransactionResponse::new();
//             Ok(response)
//             // tx_sender.send(msg).unwrap();
//             // let reply = format!("reply");
//         }
// }
