use std::cmp::max;

use time::now_utc;

use crypto::{hash, HashDigest, PublicKey, SecretKey};
use error::SimplesResult;
use simples_pb::Wallet;
use wallet::{self, WalletKeypairExt};

// #[derive(Clone, Eq, PartialEq, Debug)]
pub struct BlockTemplate {
    pub proof_hash: HashDigest,
    pub previous_block: HashDigest,
    pub timestamp: i64,
    pub staker_pk: PublicKey,
    pub staker_sk: SecretKey
}

pub const STAKING_INTERVAL: i64 = 1;

pub fn compute_proof_hash(
    head_block: &HashDigest, public_key: &PublicKey) -> HashDigest {
    let mut proof_bytes = head_block.0.to_vec();
    proof_bytes.push_all(&public_key.0);
    hash(&proof_bytes)
}

pub struct Staker {
    head_block: HashDigest,
    target_hash: HashDigest,
    target_step: HashDigest,
    untried_timestamp: i64,
    staking_keys: Wallet,
}

impl Staker {
    pub fn new(staking_keys: Wallet, head_block: HashDigest,
               head_timestamp: i64) -> Staker {
        let mut step = HashDigest::from_u64(0);
        step.0[step.0.len() - 1] = 3;
        Staker {
            head_block: head_block,
            target_hash: HashDigest::from_u64(0),
            target_step: step,
            untried_timestamp: head_timestamp + 1,
            staking_keys: staking_keys
        }
    }

    pub fn set_head_block(&mut self, head_block: HashDigest, head_timestamp: i64) {
        if head_block != self.head_block {
            self.head_block = head_block;
            self.untried_timestamp = head_timestamp + 1;
            self.target_hash = self.target_step.clone();
        }
    }

    pub fn stake_interval(&mut self, staking_interval: i64)
                          -> SimplesResult<Option<BlockTemplate>> {
        let max_timestamp = max(self.untried_timestamp + staking_interval,
                                now_utc().to_timespec().sec + 1);
        let interval = range(self.untried_timestamp, max_timestamp);
        let proof_hashes: Vec<HashDigest> =
                self.staking_keys.get_keypairs().iter().map(|kp| {
                let proof = compute_proof_hash(
                    &self.head_block, &kp.decode_public_key().unwrap());
                proof
            }).collect();
        // println!("untried={}, max_timestmap={}", self.untried_timestamp, max_timestamp);
        for timestamp in interval {
            let mut wallet_index = 0;
            // println!("target at timestep {} is {}", timestamp, self.target_hash);
            for proof_hash in proof_hashes.iter() {
                if *proof_hash < self.target_hash {
                    let keypair = &self.staking_keys.get_keypairs()[wallet_index];
                    println!("Hoo-yeah! Successfuly staked a block with address {}",
                             wallet::pretty_format(&keypair));
                    return Ok(Some(BlockTemplate {
                        proof_hash: proof_hash.clone(),
                        previous_block: self.head_block.clone(),
                        timestamp: timestamp,
                        staker_pk: try!(keypair.decode_public_key()),
                        staker_sk: try!(keypair.decode_secret_key())
                    }))
                }
                wallet_index += 1;
            }
            if self.target_hash.0[self.target_hash.0.len() - 1] < 255 {
                self.target_hash.add_in_place(&self.target_step);
            }
        }
        self.untried_timestamp = max_timestamp;
        Ok(None)
    }
}


#[test]
fn test_staker() {
}
