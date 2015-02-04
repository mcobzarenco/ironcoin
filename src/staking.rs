use std::old_io::timer::Timer;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{JoinGuard, Thread};
use std::time::Duration;

use rustc_serialize::base64::{self, ToBase64};
use time::now_utc;

use crypto::{hash, HashDigest};
use error::{SimplesResult};

pub struct ThreadedStaker {
    block_update: Sender<Option<(HashDigest, i64)>>,
    worker_thread: Thread
}

impl ThreadedStaker {
    pub fn new(head_block: HashDigest, head_timestamp: i64) ->
        SimplesResult<ThreadedStaker> {
        let (hash_send, hash_recv) = channel();
        let mut staker = StakerWorker::new(
            head_block, head_timestamp, hash_recv);
        let worker_thread = Thread::spawn(move || { staker.run(); });
        Ok(ThreadedStaker {
            block_update: hash_send,
            worker_thread: worker_thread
        })
    }

    pub fn set_head_block(&mut self, head_block: HashDigest,
                          head_timestamp: i64) {
        self.block_update.send(Some((head_block, head_timestamp)));
    }
}

impl Drop for ThreadedStaker {
    fn drop(&mut self) {
        self.block_update.send(None);
        Thread::yield_now();
    }
}

struct StakerWorker {
    head_block: HashDigest,
    head_timestamp: i64,
    block_update: Receiver<Option<(HashDigest, i64)>>,
}

impl StakerWorker {
    fn new(head_block: HashDigest, head_timestamp: i64,
           block_update: Receiver<Option<(HashDigest, i64)>>)
           -> StakerWorker {
        StakerWorker {
            head_block: head_block,
            head_timestamp: head_timestamp,
            block_update: block_update
        }
    }

    fn run(&mut self) {
        let mut timer = Timer::new().unwrap();
        let mut timeout = timer.periodic(Duration::seconds(3));
        // let mut new_timeout = None;
        let mut new_block: Option<(HashDigest, i64)> = None;
        loop { {
            // self.* does not work in the select macro,
            // so I need to take a ref:
            let block_update = &self.block_update;
            select!(
                maybe_new_block = block_update.recv() => {
                    if maybe_new_block.is_err() ||
                        maybe_new_block.as_ref().unwrap().is_none() {
                            return;
                        }
                    new_block = maybe_new_block.unwrap();
                },
                _ = timeout.recv() => {
                    println!("Timeout time: {}!",
                             now_utc().to_timespec().sec);
                }
            );} // select
            if new_block.is_some() {
                let (head_block, head_timestamp) = new_block.unwrap();
                println!("Got new block hash: {} ({})",
                         &head_block.0[].to_base64(base64::STANDARD),
                         head_timestamp);

                self.set_head_block(head_block, head_timestamp);
                new_block = None;
            }
            // if new_timeout.is_some() {
            //     let duration = Duration::seconds(new_timeout.unwrap());
            //     timeout = timer.oneshot(duration);
            //     new_timeout = None;
            // }
        } // loop
    }

    fn set_head_block(&mut self, head_block: HashDigest,
                      head_timestamp: i64) {
        self.head_block = head_block;
        self.head_timestamp = head_timestamp;
    }
}

#[test]
fn test_worker() {
    let mut ts = ThreadedStaker::new(hash(b"hello world"), 100).unwrap();
    let mut timer = Timer::new().unwrap();
    timer.sleep(Duration::seconds(5));
    ts.set_head_block(hash(b"hello world"), now_utc().to_timespec().sec);
    timer.sleep(Duration::seconds(5));
    ts.set_head_block(hash(b"fuck you"), now_utc().to_timespec().sec);
    timer.sleep(Duration::seconds(5));
    ts.set_head_block(hash(b"fuck you2"), now_utc().to_timespec().sec);
    timer.sleep(Duration::seconds(5));
}

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
