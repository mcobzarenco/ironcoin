#![feature(slicing_syntax)]
#![allow(unstable)]
extern crate getopts;
extern crate sodiumoxide;
extern crate "rustc-serialize" as rustc_serialize;
extern crate time;
extern crate nanomsg;
extern crate protobuf;

use std::os;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::Thread;
use nanomsg::{Socket, Protocol};
use getopts::{optopt, optflag, getopts, OptGroup, usage};
use rustc_serialize::json;
use rustc_serialize::base64::{self, ToBase64};
use sodiumoxide::crypto::sign::ed25519;
use protobuf::text_format;

mod tx;
mod balance;
mod ledger;
mod simples_pb;

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
    println!("Simples is configure to stake");
    main2();
    loop {
        let tx = tx_receiver.recv().unwrap();
        println!("staker: {:?}", tx);
    }
}

fn rpc_server(endpoint_str: &str, tx_sender: Sender<simples_pb::Transaction>) {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    let mut endpoint = socket.bind(endpoint_str).unwrap();
    println!("Simples is listening at {}", endpoint_str);

    loop {
        match socket.read_to_string() {
            Ok(request) => {
                match protobuf::parse_from_bytes(request.as_bytes()) {
                    Ok(msg) => {
                        println!("Recv {:?}.", msg);
                        tx_sender.send(msg).unwrap();
                    },
                    Err(err) => {
                        println!("Server failed to receive request '{:?}'.", err);
                        let reply = format!("{:?}", err);
                        match socket.write(reply.as_bytes()) {
                            Ok(..) => {},
                            Err(err) => {
                                println!("Server failed to send reply '{:?}'.", err);
                                break
                            }
                        }
                    }
                }
            },
            Err(err) => {
                println!("Server failed to receive request '{}'.", err);
                break
            }
        }
    }
    endpoint.shutdown();
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

fn main() {
    let DEFAULT_ENDPOINT:String = String::from_str("tcp://127.0.0.1:13337");
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = &[
        optflag("h", "help", "Show this help menu."),
        optopt("b", "bind", "Endpoint where to bind.", "ENDPOINT")
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

    let rpc_endpoint = matches.opt_str("b").unwrap_or(DEFAULT_ENDPOINT);
    let (tx_sender, tx_receiver) = channel();
    let staking_thread = Thread::scoped(move || {
        do_staking(tx_receiver);
    });
    let rpc_thread = Thread::scoped(move || {
        rpc_server(&rpc_endpoint[], tx_sender);
    });
}
