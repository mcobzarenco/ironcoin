#![feature(collections)]
#![feature(core)]
#![feature(io)]
#![feature(os)]
#![feature(std_misc)]
#![feature(unsafe_destructor)]
extern crate getopts2;
extern crate nanomsg;
extern crate protobuf;
extern crate sodiumoxide;
extern crate time;
extern crate uuid;
extern crate "rustc-serialize" as rustc_serialize;

mod balance;
mod block;
mod blocktree;
mod crypto;
mod error;
mod app;
mod service;
mod simples_pb;
mod staking;
mod store;
mod tx;
mod wallet;

use std::error::Error;
use std::old_io::File;
use std::old_io::timer::sleep;
use std::os;
use std::str::FromStr;
use std::thread::Thread;
use std::time::duration::Duration;

use getopts2::Options;
use protobuf::Message;
use rustc_serialize::base64::{self, FromBase64, ToBase64};

use blocktree::{BlockTreeStore, GenesisBuilder};
use crypto::{gen_keypair, PublicKey, slice_to_pk, slice_to_sk};
use error::{SimplesResult, SimplesError};
use service::{RpcService, SimplesService};
use simples_pb::HashedBlock;
use store::RocksStore;
use wallet::{load_proto_from_file, save_proto_to_file, WalletExt};

fn send_test_transactions() {
    let mut client = app::Client::new("tcp://127.0.0.1:13337").ok().unwrap();
    let mut count = 1u32;
    let sleep_duration = Duration::milliseconds(100);

    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    loop {
        let mut tx_builder = tx::TransactionBuilder::new();
        tx_builder
            .add_transfer(&sk1, &pk1, &pk2, 1, 0)
            .add_transfer(&sk2, &pk2, &pk1, 10, 0)
            .add_transfer(&sk1, &pk1, &pk2,  1, 1)
            .set_bounty(&sk1, &pk1, 1);
        let trans = tx_builder.build().unwrap();
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

fn create_genesis_block_from_cmdline(tx_strs: &[String])
                                     -> SimplesResult<HashedBlock>
{
    let mut builder = GenesisBuilder::new();
    for tx_str in tx_strs.iter() {
        let transfer_parts: Vec<&str> = tx_str.split_str(":").collect();
        if transfer_parts.len() != 2 {
            return Err(SimplesError::new(
                "A genesis transfer needs to be specified as ADDR:AMOUNT"));
        };

        let maybe_destination =
            slice_to_pk(&try!(FromBase64::from_base64(transfer_parts[0]))[]);
        if maybe_destination.is_err() {
            return Err(SimplesError::new(&format!(
                "Could not parse \"{}\" as an address.", transfer_parts[0])[]));
        }
        let destination = maybe_destination.unwrap();

        let maybe_amount = FromStr::from_str(transfer_parts[1]);
        if maybe_amount.is_err() {
            return Err(SimplesError::new(&format!(
                "Could not parse \"{}\" as an amount.", transfer_parts[1])[]));
        }
        let amount: u64 = maybe_amount.unwrap();
        builder.add_transfer(destination, amount);
    }
    Ok(builder.build())
}

fn create_block_store(block_db: &str, genesis: Option<&HashedBlock>) ->
    SimplesResult<BlockTreeStore<RocksStore>> {
    BlockTreeStore::new(try!(RocksStore::new(block_db)), genesis)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(brief.as_slice()));
}

const DEFAULT_ENDPOINT:&'static str = "tcp://127.0.0.1:13337";

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let daemon_description = format!(
        "Daemonize, run simples in the background. Pass endpoint where
        to bind (default={})", DEFAULT_ENDPOINT);
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help menu.");
    opts.optflagopt("d", "", &daemon_description[], "ENDPOINT");
    opts.optflag("", "test", "Send random transactions.");
    opts.optopt("f", "", "Load wallet file.", "FILE");
    opts.optmulti("t", "", "Specify a transfer.", "SRC:DEST:AMOUNT:OP_NUM");
    opts.optmulti("p", "", "Specify a peer.", "ENDPOINT");
    opts.optopt("", "add", "Add a pre-existing address to the wallet.",
                "[NAME:]ADDRESS|SKEY");
    opts.optflagopt("", "new", "Create and add a new address to the wallet.",
                    "[NAME]");
    opts.optflagopt("", "ls", "List all addresses contained by the wallet.",
                    "[PATTERN]");
    opts.optopt("g", "", "Set genesis block from file.", "PATH");
    opts.optopt("", "blocktree", "Specify blocktree database.", "PATH");
    opts.optopt("", "balance-db", "Specify balance database.", "PATH");
    opts.optopt("", "new-genesis", "Create a genesis block and write it to file.
Use multiple times to specify genesis transactions.", "PATH");
    opts.optmulti("", "gtx", "Use with --create-genesis. The argument can be
used multiple times to specify genesis transactions.", "ADDR:AMOUNT");
    let matches = match opts.parse(args.tail()) {
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
        Thread::scoped(move || {
            send_test_transactions();
        });
    }
    if matches.opt_present("f") {
        let wallet_file = matches.opt_str("f").unwrap();
        println!("Reading wallet from file: {}", wallet_file);
        wallet = wallet::load_from_file(&wallet_file[]).unwrap();

        if matches.opt_present("new") {
            let key = match matches.opt_str("new") {
                Some(name) => wallet.generate_new_key(&name[]),
                None => {
                    let name = wallet.generate_name();
                    wallet.generate_new_key(&name[])
                }};
            println!("Created new address: {}",
                     key.get_public_key().to_base64(base64::STANDARD));
            wallet::save_to_file(&wallet_file[], &wallet).unwrap()
        }
        if matches.opt_present("ls") {
            let pattern = matches.opt_str("ls").unwrap_or(String::new());
            if &pattern[] != "" {
                let _: Vec<()> = wallet.search_keys(&pattern[]).iter()
                    .map(|wkey| { println!("{}", wallet::pretty_format(*wkey)); })
                    .collect();
            } else {
                let _: Vec<()> = wallet.get_keys().iter().map(
                    |wkey| println!("{}", wallet::pretty_format(wkey))).collect();
            }
        }
    }
    if matches.opt_present("t") {
        let mut tx_builder = tx::TransactionBuilder::new();
        for transfer_str in matches.opt_strs("t").iter() {
            let transfer_parts: Vec<&str> =
                transfer_str.split_str(":").collect();
            if transfer_parts.len() != 4 {
                println!("You need to specify the transfer you wish
 to make as SOURCE:DESTINATION:AMOUNT:OP_NUM.");
            return;
            };

            let source = transfer_parts[0];
            let destination = transfer_parts[1];
            let amount: u64 = FromStr::from_str(transfer_parts[2]).unwrap();
            let op_number: u32 = FromStr::from_str(transfer_parts[3]).unwrap();

            let source_keys = wallet.search_keys(&source[]);
            let destination_keys = wallet.search_keys(&destination[]);
            if source_keys.len() == 0 {
                println!(
                    "The wallet doesn't contain the source address \"{}\"",
                    source);
                return;
            } else if destination_keys.len() == 0{
                println!(
                    "The wallet doesn't contain the destination address \"{}\"",
                    destination);
                return;
            } else if source_keys.len() > 1 || destination_keys.len() > 1 {
                println!(
                    "The wallet contains multiple addresses that match \"{}\":",
                    source);
                let _: Vec<()> = source_keys.iter().map(|wkey| {
                    println!("{}", wallet::pretty_format(*wkey))}).collect();
                return;
            }

            let source_sk =
                slice_to_sk(source_keys[0].get_secret_key()).unwrap();
            let source_pk =
                slice_to_pk(source_keys[0].get_public_key()).unwrap();
            let destination_pk =
                slice_to_pk(destination_keys[0].get_public_key()).unwrap();
            tx_builder
                .add_transfer(&source_sk, &source_pk, &destination_pk,
                              amount, op_number)
                .set_bounty(&source_sk, &source_pk, 1);
        }
        let transaction = tx_builder.build().unwrap();
        println!("{}", protobuf::text_format::print_to_string(&transaction));
        let mut client = app::Client::new("tcp://127.0.0.1:13337").unwrap();
        let mut request = simples_pb::PublishTransactionRequest::new();
        request.set_transaction(transaction);
        let response = client.pub_transaction(request).ok().unwrap();
        println!("Response status: {:?}", response);
    }
    if matches.opt_present("new-genesis") {
        let genesis_path = matches.opt_str("new-genesis").unwrap();
        println!("Writing new genesis block to \"{}\"", genesis_path);
        let maybe_genesis =
            create_genesis_block_from_cmdline(&matches.opt_strs("gtx")[]);
        if maybe_genesis.is_err() {
            println!("{}", maybe_genesis.unwrap_err());
            return;
        }
        let genesis_block = maybe_genesis.unwrap();
        let mut genesis_out = File::create(&Path::new(genesis_path.clone()));
        let genesis_bytes = genesis_block.write_to_bytes().unwrap();
        let write_result = genesis_out.write_all(&genesis_bytes[]);
        if write_result.is_err() {
            println!("Could not write genesis to file \"{}\"", genesis_path);
            return;
        }
    }
        // match matches.opt_str("genesis") {
        //     Some(path) => {
        //         let genesis_out = File::open(&Path::new(path));
        //         let genesis_bytes =
        //             block_db.get_genesis().write_to_bytes().unwrap();
        //         let write_result = genesis_out.write_all(&genesis_bytes[]);
        //         if write_result.is_err() {
        //             println!("Could not write genesis to file \"{}\"", path);
        //             return;
        //         }},
        //     None => {}
        // }
    if matches.opt_present("d") {
        let blocktree_path = matches.opt_str("blocktree")
            .unwrap_or(String::from_str("block.rdb"));
        let balance_db = matches.opt_str("balance-db")
            .unwrap_or(String::from_str("balance.rdb"));

        let genesis_block: Option<HashedBlock> = match matches.opt_str("g") {
            Some(path) => {
                let maybe_genesis = load_proto_from_file(&path[]);
                if maybe_genesis.is_err() {
                    println!("ERROR: Could not read genesis block from file {}", path);
                    return;
                }
                Some(maybe_genesis.unwrap())
            },
            None => None
        };
        let blocktree;
        match create_block_store(&blocktree_path[], genesis_block.as_ref()) {
            Ok(inner_blocktree) => { blocktree = inner_blocktree; },
            Err(err) => {
                println!("FATAL: Could not instantiate a blocktree from {}.\n{}",
                         blocktree_path, err);
                return;
            }
        };

        let mut peers = vec![];
        if matches.opt_present("p") {
            peers.push_all(&matches.opt_strs("p"));
        }

        let service =
            SimplesService::new(&balance_db[], blocktree).unwrap();
        let mut app =
            app::Application::new(&rpc_endpoint[], service, peers, wallet).unwrap();
        app.run().map_err(|err| {
            println!("App existed with error '{}'", err.description());
        });
    }
}
