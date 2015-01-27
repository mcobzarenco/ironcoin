#![feature(slicing_syntax)]
#![feature(unsafe_destructor)]
#![allow(unstable)]
extern crate getopts;
extern crate sodiumoxide;
extern crate "rustc-serialize" as rustc_serialize;
extern crate time;
extern crate nanomsg;
extern crate protobuf;

mod balance;
mod error;
mod rpc;
mod service;
mod simples_pb;
mod tx;

use std::os;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::duration::Duration;
use std::io::timer::sleep;
use std::thread::Thread;

use getopts::{optopt, optflag, getopts, OptGroup, usage};
use nanomsg::{Socket, Protocol};
use protobuf::Message;
use protobuf::text_format;
use sodiumoxide::crypto::sign::ed25519;

use error::{SimplesError, SimplesResult};
use service::{SimplesService};

fn main2() {
    let (pk1, sk1) = ed25519::gen_keypair();
    let (pk2, sk2) = ed25519::gen_keypair();

    let trans = tx::TransactionBuilder::new()
        .add_transfer(&sk1, &pk1, &pk2, 98, 0)
        .add_transfer(&sk2, &pk2, &pk1, 10, 0)
        .add_transfer(&sk1, &pk1, &pk2,  1, 1)
        .set_bounty(&sk1, &pk1, 1)
        .build().unwrap();

    let encoded:String = text_format::print_to_string(&trans);
    // println!("encoded: {:?}", encoded.as_bytes().to_base64(base64::STANDARD));
    println!("encoded: {:?}", encoded);

    // let sm:Vec<u8> = crypto::sign::ed25519::sign(b"ma-ta", &sk);
    // match crypto::sign::ed25519::verify(sm.as_slice(), &pk) {
    //     Some(m) =>
    //         println!("Valid signature: {} ", String::from_utf8(m).unwrap()),
    //     None => println!("Invalid signature.")
    // }

    let store = balance::Store::new("state.rdb");
    let mut cache = store.mutate();
    cache.add_transaction(&trans).unwrap();

    // let db = RocksDB::open_default("test.rdb").unwrap();
    // db.put(b"my key2", b"das");
    // db.get(b"my key")
    //     .map(|value| {
    //         println!("retrieved value: {}", value.to_utf8().unwrap());
    //     })
    //     .on_absent( || { println!("value not found") })
    //     .on_error( |e| { println!("operational problem encountered: {}", e) });

    // db.close();

    // println!("us: {}", us.as_slice().to_base64(base64::STANDARD));
    // println!("pk, sk: {}, {}",
    //          pk1.as_slice().to_base64(base64::STANDARD),
    //          sk1.as_slice().to_base64(base64::STANDARD));
}

fn do_staking(tx_receiver: Receiver<simples_pb::Transaction>) {
    println!("Simples is configured to stake");
    loop {
        let tx = tx_receiver.recv().unwrap();
        println!("staker: {:?}", tx);
    }
}

struct TestService {
    tx_sender: Sender<simples_pb::Transaction>,
}

impl service::SimplesService for TestService {
    fn pub_transaction(
        &mut self, request: simples_pb::PublishTransactionRequest) ->
        SimplesResult<simples_pb::PublishTransactionResponse> {
            println!("{:?}", request.get_transaction());
            let response = simples_pb::PublishTransactionResponse::new();
            Ok(response)
            // tx_sender.send(msg).unwrap();
            // let reply = format!("reply");
        }
}

fn send_test_transactions() {
    let mut client = rpc::Client::new("tcp://127.0.0.1:13337").ok().unwrap();
    let mut count = 1u32;
    let sleep_duration = Duration::milliseconds(100);

    let (pk1, sk1) = ed25519::gen_keypair();
    let (pk2, sk2) = ed25519::gen_keypair();
    loop {
        let trans = tx::TransactionBuilder::new()
            .add_transfer(&sk1, &pk1, &pk2, 1, 0)
            .add_transfer(&sk2, &pk2, &pk1, 1, 0)
            .add_transfer(&sk1, &pk1, &pk2,  1, 1)
            .set_bounty(&sk1, &pk1, 1)
            .build().unwrap();

        let mut request = simples_pb::PublishTransactionRequest::new();
        request.set_transaction(trans);

        println!("Sending random transaction number {}.", count);
        let response = client.pub_transaction(request).ok().unwrap();
        println!("Got response");
        println!("{:?}", response);

        // request.set_method(simples_pb::RpcRequest_Method::PUBLISH_TRANSACTION);

        // println!("getting bytes");
        // let trans_bytes = request.write_to_bytes().unwrap();

        // match socket.write(&trans_bytes[]) {
        //     Ok(..) => println!("Sent transaction num {}.", count),
        //     Err(err) => {
        //         println!("Client failed to send request '{}'.", err);
        //         break
        //     }
        // }

        // match socket.read_to_string() {
        //     Ok(reply) => println!("Recv '{}'.", reply.as_slice()),
        //     Err(err) => {
        //         println!("Client failed to receive reply '{}'.", err);
        //         break
        //     }
        // }

        sleep(sleep_duration);
        count += 1;
    }
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

const DEFAULT_ENDPOINT:&'static str = "tcp://127.0.0.1:13337";

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = &[
        optflag("h", "help", "Show this help menu."),
        optopt("b", "bind", "Endpoint where to bind.", "ENDPOINT"),
        optflag("t", "", "Send random transactions."),
        optflag("g", "", "Send random transactions.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}", f);
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    let rpc_endpoint = matches.opt_str("b")
        .unwrap_or(String::from_str(DEFAULT_ENDPOINT));
    let (tx_sender, tx_receiver) = channel();
    let service = TestService { tx_sender: tx_sender };
    let mut app = rpc::Application::new(&rpc_endpoint[], service)
        .ok().unwrap();
    if matches.opt_present("t") {
        let client_thread = Thread::scoped(move || {
            send_test_transactions();
        });
    } else {
        // let staking_thread = Thread::scoped(move || {
        //     do_staking(tx_receiver);
        // });
        let rpc_thread = Thread::scoped(move || {
            app.run().map_err(|err| {
                println!("App existed with error '{}'", err.description);
            });
        });
    }
}
