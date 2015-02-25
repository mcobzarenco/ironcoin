#![feature(collections)]
#![feature(core)]
#![feature(env)]
#![feature(fs)]
#![feature(int_uint)]
#![feature(io)]
#![feature(path)]
#![feature(std_misc)]
#![feature(unsafe_destructor)]
extern crate getopts;
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

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use getopts::Options;
use protobuf::Message;
use rustc_serialize::base64::{self, FromBase64, ToBase64};

use block::GenesisBuilder;
use blocktree::BlockTreeStore;
use crypto::PublicKey;
use error::{SimplesResult, SimplesError};
use service::{RpcService, SimplesService};
use simples_pb::HashedBlock;
use store::RocksStore;
use wallet::{load_proto_from_file, WalletExt, WalletKeypairExt};

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
            PublicKey::from_slice(&try!(FromBase64::from_base64(transfer_parts[0])));
        if maybe_destination.is_err() {
            return Err(SimplesError::new(&format!(
                "Could not parse \"{}\" as an address.", transfer_parts[0])));
        }
        let destination = maybe_destination.unwrap();

        let maybe_amount = FromStr::from_str(transfer_parts[1]);
        if maybe_amount.is_err() {
            return Err(SimplesError::new(&format!(
                "Could not parse \"{}\" as an amount.", transfer_parts[1])));
        }
        let amount: u64 = maybe_amount.unwrap();
        builder.add_transfer(destination, amount);
    }
    Ok(builder.build())
}

fn create_block_store(block_db: &str, genesis: Option<HashedBlock>) ->
    SimplesResult<BlockTreeStore<RocksStore>> {
    BlockTreeStore::new(try!(RocksStore::new(block_db)), genesis)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(brief.as_slice()));
}

const DEFAULT_ENDPOINT:&'static str = "tcp://127.0.0.1:13337";

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let daemon_description = format!(
        "Daemonize, run simples in the background. Pass endpoint where
        to bind (default={})", DEFAULT_ENDPOINT);
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help menu.");
    opts.optflagopt("d", "", &daemon_description, "ENDPOINT");
    opts.optopt("f", "", "Load wallet file.", "FILE");
    opts.optmulti("t", "", "Specify a transfer.", "SRC:DEST:AMOUNT:OP_NUM");
    opts.optmulti("p", "", "Specify a peer.", "ENDPOINT");
    opts.optopt("", "add", "Add a pre-existing address to the wallet.",
                "[NAME:]ADDR|SKEY");
    opts.optflagopt("", "new", "Create and add a new address to the wallet.",
                    "NAME");
    opts.optflagopt("", "ls", "List all addresses contained by the wallet.",
                    "PATTERN");
    opts.optopt("g", "", "Set genesis block from file.", "PATH");
    opts.optopt("", "blocktree", "Specify blocktree database.", "PATH");
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
    if matches.opt_present("f") {
        let wallet_file = matches.opt_str("f").unwrap();
        println!("Reading wallet from file: {}", wallet_file);
        wallet = wallet::load_from_file(&wallet_file).unwrap();

        if matches.opt_present("new") {
            let key = match matches.opt_str("new") {
                Some(name) => wallet.generate_new_key(&name),
                None => {
                    let name = wallet.generate_name();
                    wallet.generate_new_key(&name)
                }};
            println!("Created new address: {}",
                     key.get_public_key().to_base64(base64::STANDARD));
            wallet::save_to_file(&wallet_file, &wallet).unwrap()
        }
        if matches.opt_present("ls") {
            let pattern = matches.opt_str("ls").unwrap_or(String::new());
            if &pattern[..] != "" {
                let _: Vec<()> = wallet.search_keys(&pattern).iter()
                    .map(|wkey| { println!("{}", wallet::pretty_format(*wkey)); })
                    .collect();
            } else {
                let _: Vec<()> = wallet.get_keypairs().iter().map(
                    |wkey| println!("{}", wallet::pretty_format(wkey))).collect();
            }
        }
        if matches.opt_present("add") {
            let addr_pattern = matches.opt_str("add").unwrap_or(String::new());
            let addr_parts: Vec<&str> = addr_pattern.split_str(":").collect();
            if addr_parts.len() != 2 {
                println!("You need to specify the address you wish \
                          to add as NAME:PKEY.");
                return;
            }
            let addr_name = addr_parts[0];
            let public_key = PublicKey::from_slice(
                &FromBase64::from_base64(addr_parts[1]).unwrap()).unwrap();
            wallet.add_public_key(&addr_name, &public_key);
            wallet::save_to_file(&wallet_file, &wallet).unwrap();
        }
    }
    let mut peers = vec![];
    if matches.opt_present("p") {
        peers.push_all(&matches.opt_strs("p"));
    }
    if matches.opt_present("t") {
        let mut tx_builder = tx::TransactionBuilder::new();
        for transfer_str in matches.opt_strs("t").iter() {
            let transfer_parts: Vec<&str> =
                transfer_str.split_str(":").collect();
            if transfer_parts.len() != 4 {
                println!("You need to specify the transfer you wish \
                          to make as SOURCE:DESTINATION:AMOUNT:OP_NUM.");
                return;
            }
            let source = transfer_parts[0];
            let destination = transfer_parts[1];
            let amount: u64 = FromStr::from_str(transfer_parts[2]).unwrap();
            let op_number: u32 = FromStr::from_str(transfer_parts[3]).unwrap();

            let source_keys = wallet.search_keys(&source);
            let destination_keys = wallet.search_keys(&destination);
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
            let source_sk = source_keys[0].decode_secret_key().unwrap();
            let source_pk = source_keys[0].decode_public_key().unwrap();
            let destination_pk = destination_keys[0].decode_public_key().unwrap();
            tx_builder
                .add_transfer(&source_sk, &source_pk, &destination_pk,
                              amount, op_number)
                .set_bounty(&source_sk, &source_pk, 1);
        }
        let transaction = tx_builder.build().unwrap();
        if peers.len() == 0 {
            println!("ERROR: No peers were specified.");
            return;
        }
        for peer in peers.iter() {
            let mut client = service::Client::new(peer).unwrap();
            let mut request = simples_pb::PubTransactionRequest::new();
            request.set_transaction(transaction.clone());
            let response = client.pub_transaction(request).ok().unwrap();
            println!("Response status: {:?}", response);
        }
    }
    if matches.opt_present("new-genesis") {
        let genesis_path = matches.opt_str("new-genesis").unwrap();
        println!("Writing new genesis block to \"{}\"", genesis_path);
        let maybe_genesis =
            create_genesis_block_from_cmdline(&matches.opt_strs("gtx"));
        if maybe_genesis.is_err() {
            println!("{}", maybe_genesis.unwrap_err());
            return;
        }
        let genesis_block = maybe_genesis.unwrap();
        let mut genesis_out =
            File::create(&Path::new(&genesis_path)).unwrap();
        let genesis_bytes = genesis_block.write_to_bytes().unwrap();
        let write_result = genesis_out.write_all(&genesis_bytes);
        if write_result.is_err() {
            println!("Could not write genesis to file \"{}\"", genesis_path);
            return;
        }
    }
    if matches.opt_present("d") {
        let blocktree_path = matches.opt_str("blocktree")
            .unwrap_or(String::from_str("block.rdb"));

        let genesis_block: Option<HashedBlock> = match matches.opt_str("g") {
            Some(path) => {
                let maybe_genesis = load_proto_from_file(&path);
                if maybe_genesis.is_err() {
                    println!("ERROR: Could not read genesis block from file {}", path);
                    return;
                }
                Some(maybe_genesis.unwrap())
            },
            None => None
        };

        let blocktree;
        match create_block_store(&blocktree_path, genesis_block) {
            Ok(inner_blocktree) => { blocktree = inner_blocktree; },
            Err(err) => {
                println!("FATAL: Could not instantiate a blocktree from {}.\n{}",
                         blocktree_path, err);
                return;
            }
        };

        let service =
            SimplesService::new(blocktree).unwrap();
        let mut app =
            app::Application::new(rpc_endpoint, service, peers, wallet).unwrap();
        app.run().unwrap();
    }
}
