use std::cmp::max;
use std::mem::transmute;

use time::now_utc;

use crypto::{hash, HashDigest, PublicKey};
use error::SimplesResult;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BlockTemplate {
    pub proof_hash: HashDigest,
    pub previous_block: HashDigest,
    pub timestamp: i64
}

pub const STAKING_INTERVAL: i64 = 3;
const U64_BYTES: usize = 8;

pub struct Staker {
    head_block: HashDigest,
    untried_timestamp: i64,
    wallet_keys: Vec<PublicKey>
}

impl Staker {
    pub fn new(head_block: HashDigest, head_timestamp: i64) -> Staker {
        Staker {
            head_block: head_block,
            untried_timestamp: head_timestamp + 1,
            wallet_keys: vec![]
        }
    }

    pub fn set_head_block(&mut self, head_block: HashDigest, head_timestamp: i64) {
        self.head_block = head_block;
        self.untried_timestamp = head_timestamp + 1;
    }

    pub fn stake_interval(&mut self, staking_interval: i64) -> Option<BlockTemplate> {
        let max_timestamp = max(self.untried_timestamp + staking_interval,
                                now_utc().to_timespec().sec + 1);
        let interval = range(self.untried_timestamp, max_timestamp);
        for timestamp in interval {
            let mut proof_bytes = self.head_block.0.to_vec();
            let timestamp_bytes: [u8; U64_BYTES]
                = unsafe { transmute(timestamp) };
            proof_bytes.push_all(&timestamp_bytes[]);
            let proof_hash = hash(&proof_bytes[]);
            if proof_hash.0[0] < 15 {
                return Some(BlockTemplate {
                    proof_hash: proof_hash,
                    previous_block: self.head_block.clone(),
                    timestamp: timestamp
                });
            }
        }
        self.untried_timestamp = max_timestamp;
        None
    }
}


#[test]
fn test_staker() {
}
