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
mod staking;
mod store;
mod tx;
mod wallet;

use std::error::{Error};
use std::old_io::timer::sleep;
use std::os;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::Thread;
use std::time::duration::Duration;
use std::vec;

use getopts::{optopt, optflag, optflagopt, getopts, OptGroup, usage};
use nanomsg::{Socket};
use protobuf::Message;
use protobuf::text_format;
use rustc_serialize::base64::{self, FromBase64, ToBase64};
use sodiumoxide::crypto::sign::ed25519;

use error::{SimplesError, SimplesResult};
use service::Service;
use wallet::WalletExt;

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

fn send_test_transactions() {
    let mut client = rpc::Client::new("tcp://127.0.0.1:13337").ok().unwrap();
    let mut count = 1u32;
    let sleep_duration = Duration::milliseconds(100);

    let (pk1, sk1) = ed25519::gen_keypair();
    let (pk2, sk2) = ed25519::gen_keypair();
    loop {
        let mut trans = tx::TransactionBuilder::new()
            .add_transfer(&sk1, &pk1, &pk2, 1, 0)
            .add_transfer(&sk2, &pk2, &pk1, 10, 0)
            .add_transfer(&sk1, &pk1, &pk2,  1, 1)
            .set_bounty(&sk1, &pk1, 1)
            .build().unwrap();
        println!("Len detached signs: {}", trans.get_signatures().len());
        // trans.mut_signatures().pop();
        // trans.mut_signatures().pop();
        // println!("Len detached signs: {}", trans.get_signatures().len());

        let mut request = simples_pb::PublishTransactionRequest::new();
        request.set_transaction(trans);

        println!("Req has transaction {}.",
                 request.has_transaction());
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
    let daemon_description = format!(
        "Daemonize, run simples in the background. Pass endpoint where
        to bind (default={})", DEFAULT_ENDPOINT);
    let opts = &[
        optflag("h", "help", "Show this help menu."),
        optflagopt("d", "", &daemon_description[], "ENDPOINT"),
        optflag("", "test", "Send random transactions."),
        optopt("f", "", "Load wallet file.", "FILE"),
        optopt("t", "", "Specify a transfer.", "SRC:DEST:AMOUNT:OP_NUM"),
        optopt("", "new", "Create and add a new address to the wallet.",
               "[NAME[:DESC]]"),
        optflagopt("", "ls", "List all addresses contained by the wallet.",
               "[PATTERN]"),
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

    let rpc_endpoint = matches.opt_str("d")
        .unwrap_or(String::from_str(DEFAULT_ENDPOINT));
    let mut wallet = simples_pb::Wallet::new();
    if matches.opt_present("test") {
        let client_thread = Thread::scoped(move || {
            send_test_transactions();
        });
    }
    if matches.opt_present("f") {
        let wallet_file = matches.opt_str("f").unwrap();
        println!("Reading wallet from file: {}", wallet_file);
        wallet = wallet::load_from_file(&wallet_file[]).unwrap();

        if matches.opt_present("new") {
            let name_desc_str = matches.opt_str("new").unwrap();
            let name_desc:Vec<&str> = name_desc_str.split_str(":").collect();
            if name_desc.len() != 2 {
                println!("Specify [NAME:[DESCRIPTION]] for the new address.");
                return;
            }
            let key = wallet.generate_new_key(name_desc[0], name_desc[1]);
            println!("Created new address: {}",
                     key.get_public_key().to_base64(base64::STANDARD));
            wallet::save_to_file(&wallet_file[], &wallet).unwrap()
        }
        if matches.opt_present("ls") {
            let pattern = matches.opt_str("ls").unwrap_or(String::new());
            if &pattern[] != "" {
                let _: Vec<()> = wallet.get_keys_by_name(&pattern[]).iter()
                    .map(|wkey| { println!("{}", wallet::pretty_format(*wkey)); })
                    .collect();
            } else {
                let _: Vec<()> = wallet.get_keys().iter().map(
                    |wkey| println!("{}", wallet::pretty_format(wkey))).collect();
            }
        }
    }
    if matches.opt_present("t") {
        let transfer_str = matches.opt_str("t").unwrap();
        let transfer_parts: Vec<&str> = transfer_str.split_str(":").collect();
        if transfer_parts.len() != 4 {
            println!("You need to specify the transfer you wish
 to make as SOURCE:DESTINATION:AMOUNT:OP_NUM.");
            return;
        };

        let source = transfer_parts[0].from_base64().unwrap();
        let destination = transfer_parts[1].from_base64().unwrap();
        let amount: u64 = FromStr::from_str(transfer_parts[2]).unwrap();
        let op_number: u32 = FromStr::from_str(transfer_parts[3]).unwrap();

        let source_key: Vec<&simples_pb::WalletKey> = wallet.get_keys().iter()
            .filter(|wkey| wkey.get_public_key() == source).collect();
        if source_key.len() == 0 {
            println!("The wallet doesn't have the secret key for source address {}",
                     transfer_parts[0]);
            return;
        }
        let secret_key = source_key[0].get_secret_key();
        let transaction = tx::TransactionBuilder::new()
            .add_transfer(&tx::slice_to_sk(secret_key).unwrap(),
                          &tx::slice_to_pk(&source[]).unwrap(),
                          &tx::slice_to_pk(&destination[]).unwrap(),
                          amount, op_number)
            .set_bounty(&tx::slice_to_sk(secret_key).unwrap(),
                        &tx::slice_to_pk(&source[]).unwrap(), 1)
            .build().unwrap();

        let mut client = rpc::Client::new("tcp://127.0.0.1:13337").ok().unwrap();
        let mut request = simples_pb::PublishTransactionRequest::new();
        request.set_transaction(transaction);
        let response = client.pub_transaction(request).ok().unwrap();
        println!("Response status: {:?}", response);
    }
    if matches.opt_present("d") {
        // let staking_thread = Thread::scoped(move || {
        //     do_staking(tx_receiver);
        // });
        let rpc_thread = Thread::scoped(move || {
            let service = service::SimplesService::new("balance.rdb").unwrap();
            let mut app =
                rpc::Application::new(&rpc_endpoint[], service).unwrap();
            app.run().map_err(|err| {
                println!("App existed with error '{}'", err.description());
            }).unwrap();
        });
    }
}
