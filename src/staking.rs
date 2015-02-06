use std::cmp::max;
use std::error::FromError;
use std::mem::transmute;
use std::old_io::timer::Timer;
use std::thread::Thread;
use std::time::Duration;

use nanomsg::{Endpoint, NanoErrorKind, NanoResult, PollRequest, Protocol, Socket};
use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::json;
use time::now_utc;
use uuid::Uuid;

use crypto::{hash, HashDigest, PublicKey};
use error::{SimplesResult};

#[derive(Clone, Eq, PartialEq, Debug, RustcEncodable, RustcDecodable)]
struct BlockSummary {
    hash: HashDigest,
    previous_block: HashDigest,
    timestamp: i64
}

#[derive(Clone, Eq, PartialEq, Debug, RustcEncodable, RustcDecodable)]
pub struct BlockTemplate {
    proof_hash: HashDigest,
    previous_block: HashDigest,
    timestamp: i64
}

pub struct ThreadedStaker {
    set_head_block_address: String,
    head_block_updater: HeadBlockUpdater,
    pub get_staked_block: Socket,
    get_staked_block_endpoint: Endpoint,
    worker_thread: Thread
}

impl ThreadedStaker {
    pub fn new() -> SimplesResult<ThreadedStaker> {
        let set_head_block_address =
            format!("inproc://{}", Uuid::new_v4().to_string());
        let head_block_updater =
            try!(HeadBlockUpdater::new(&set_head_block_address[]));

        let get_address = format!("inproc://{}", Uuid::new_v4().to_string());
        let mut get_staked_block = try!(Socket::new(Protocol::Pull));
        let get_staked_block_endpoint =
            try!(get_staked_block.bind(&get_address[]));

        let mut staker = try!(
            StakingWorker::new(&set_head_block_address[], &get_address[]));
        let worker_thread = Thread::spawn(move || { staker.run(); });
        Ok(ThreadedStaker {
            set_head_block_address: set_head_block_address,
            head_block_updater: head_block_updater,
            get_staked_block: get_staked_block,
            get_staked_block_endpoint: get_staked_block_endpoint,
            worker_thread: worker_thread
        })
    }

    pub fn new_head_block_updater(&self) -> SimplesResult<HeadBlockUpdater> {
        Ok(try!(HeadBlockUpdater::new(&self.set_head_block_address[])))
    }

    pub fn set_head_block(&mut self, head_block: HashDigest,
                          previous_block: HashDigest, head_timestamp: i64)
                          -> SimplesResult<()> {
        self.head_block_updater.set_head_block(
            head_block, previous_block, head_timestamp)
    }
}

impl Drop for ThreadedStaker {
    fn drop(&mut self) {
        let none: Option<BlockSummary> = None;
        let none_enc = json::encode(&none).unwrap();
        if self.head_block_updater.set_head_block.write_all(
            none_enc.as_bytes()).is_ok() {
            Thread::yield_now();
        }
        self.get_staked_block_endpoint.shutdown().unwrap();
    }
}

pub struct HeadBlockUpdater {
    set_head_block: Socket,
    set_head_block_endpoint: Endpoint
}

impl HeadBlockUpdater {
    fn new(address: &str) -> SimplesResult<HeadBlockUpdater> {
        let mut set_head_block = try!(Socket::new(Protocol::Push));
        let set_head_block_endpoint = try!(set_head_block.connect(address));
        Ok(HeadBlockUpdater {
            set_head_block: set_head_block,
            set_head_block_endpoint: set_head_block_endpoint
        })
    }

    pub fn set_head_block(&mut self, head_block: HashDigest,
                          previous_block: HashDigest, head_timestamp: i64)
                          -> SimplesResult<()> {
        let summary = Some(BlockSummary {
            hash: head_block,
            previous_block: previous_block,
            timestamp: head_timestamp
        });
        let summary_enc = json::encode(&summary).unwrap();
        try!(self.set_head_block.write_all(summary_enc.as_bytes()));
        Ok(())
    }
}

impl Drop for HeadBlockUpdater {
    fn drop(&mut self) {
        self.set_head_block_endpoint.shutdown().unwrap();
    }
}

const STAKING_INTERVAL: i64 = 3;

struct StakingWorker {
    listen_new_block: Socket,
    listen_new_block_endpoint: Endpoint,
    publish_address: String,
    publish_staked: Socket,
    publish_staked_endpoint: Endpoint,
    head_block_and_time: Option<(BlockSummary, i64)>,
    staked_block: Option<BlockTemplate>,
    wallet_keys: Vec<PublicKey>
}

impl StakingWorker {
    fn new(listen_address: &str, publish_address: &str)
           -> SimplesResult<StakingWorker> {
        let mut listen_new_block = try!(Socket::new(Protocol::Pull));
        let listen_new_block_endpoint =
            try!(listen_new_block.bind(listen_address));
        let mut publish_staked = try!(Socket::new(Protocol::Push));
        let publish_staked_endpoint =
            try!(publish_staked.connect(publish_address));

        Ok(StakingWorker {
            listen_new_block: listen_new_block,
            listen_new_block_endpoint: listen_new_block_endpoint,
            publish_address: String::from_str(publish_address),
            publish_staked: publish_staked,
            publish_staked_endpoint: publish_staked_endpoint,
            head_block_and_time: None,
            staked_block: None,
            wallet_keys: vec![]
        })
    }

    fn run(&mut self) -> SimplesResult<()> {
        let mut poll_fd = [self.listen_new_block.new_pollfd(true, false)];
        let mut poll_request = PollRequest::new(&mut poll_fd[]);
        loop {
            if self.staked_block.is_none() {
                let maybe_staked = self.try_staking(STAKING_INTERVAL);
                if maybe_staked.is_some() {
                    println!("Successfully staked a new block:\n{:?}",
                             maybe_staked.as_ref().unwrap());
                    let staked_enc = json::encode(
                        maybe_staked.as_ref().unwrap()).unwrap();
                    self.publish_staked.write_all(staked_enc.as_bytes()).unwrap();
                    self.staked_block = maybe_staked;
                    println!("Sent block!");
                }
            }
            match Socket::poll(&mut poll_request,
                               &Duration::seconds(STAKING_INTERVAL)) {
                Ok(_) => {
                    let msg = try!(self.listen_new_block.read_to_end());
                    let msg_as_utf8 = ::std::str::from_utf8(&msg[]).unwrap();
                    let maybe_new_block: Option<BlockSummary> =
                        json::decode(msg_as_utf8).unwrap();
                    if maybe_new_block.is_none() {
                        println!("Shutting down staking worker at {}",
                                 self.publish_address);
                        return Ok(());
                    }
                    let new_block  = maybe_new_block.unwrap();
                    println!("Got new block hash: {} ({})",
                             new_block.hash, new_block.timestamp);
                    self.set_head_block(new_block);
                },
                Err(err) => {
                    if err.kind != NanoErrorKind::Timeout {
                        println!("StakingWorker: Failed to read in message: '{}'.",
                                 err);
                        return Err(FromError::from_error(err));
                    } else {
                        println!("Timeout: {}", now_utc().to_timespec().sec);
                    }
                }
            };
        } // loop
    }

    fn set_head_block(&mut self, head_block: BlockSummary) {
        let untried_timestamp = head_block.timestamp + 1;
        self.head_block_and_time = Some((head_block, untried_timestamp));
        self.staked_block = None;
    }

    fn try_staking(&mut self, staking_interval: i64) -> Option<BlockTemplate> {
        if self.head_block_and_time.is_none() {
            return None;
        }
        let &mut (ref head_block, ref mut untried_timestamp) =
            self.head_block_and_time.as_mut().unwrap();
        let max_timestamp = max(*untried_timestamp + staking_interval,
                                now_utc().to_timespec().sec + 1);
        let interval = range(*untried_timestamp, max_timestamp);
        for timestamp in interval {
            let mut proof_bytes = head_block.hash.0.to_vec();
            let timestamp_bytes: [u8; 8] = unsafe { transmute(timestamp) };
            proof_bytes.push_all(&timestamp_bytes[]);
            let proof_hash = hash(&proof_bytes[]);
            println!("{}, timestamp={}, transmuted={:?}",
                     proof_hash.0[0], timestamp, timestamp_bytes);
            if proof_hash.0[0] < 30 {
                println!("Staked {}", &proof_hash.0[].to_base64(base64::STANDARD));
                return Some(BlockTemplate {
                    proof_hash: proof_hash,
                    previous_block: head_block.hash.clone(),
                    timestamp: timestamp
                });
            }
        }
        *untried_timestamp = max_timestamp;
        None
    }
}

impl Drop for StakingWorker {
    fn drop(&mut self) {
        self.listen_new_block_endpoint.shutdown().unwrap();
        self.publish_staked_endpoint.shutdown().unwrap();
    }
}

#[test]
fn test_uuid() {
    let uuid1 = Uuid::new_v4();
    println!("{}", uuid1.to_string());
}

#[test]
fn test_worker() {
    let mut ts = ThreadedStaker::new().unwrap();
    let mut updater = ts.new_head_block_updater().unwrap();
    updater.set_head_block(hash(b"hello world"), HashDigest::from_u64(0),
                           now_utc().to_timespec().sec);

    let mut timer = Timer::new().unwrap();
    timer.sleep(Duration::seconds(2));
    updater.set_head_block(hash(b"hello world2"), hash(b"hello world"),
                      now_utc().to_timespec().sec);
    timer.sleep(Duration::seconds(5));
    // updater.set_head_block(hash(b"fuck you"), hash(b"hello world2"),
    //                   now_utc().to_timespec().sec);
    // timer.sleep(Duration::seconds(30));
    // updater.set_head_block(hash(b"fuck you2"), hash(b"fuck you"),
    //                   now_utc().to_timespec().sec);
    // timer.sleep(Duration::seconds(60));
    // updater.set_head_block(hash(b"fuck you3"), hash(b"fuck you2"),
    //                   now_utc().to_timespec().sec);
    // timer.sleep(Duration::seconds(60));
}
