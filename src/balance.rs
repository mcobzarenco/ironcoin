extern crate rocksdb;

use std::collections::hash_map::{self, HashMap};
use std::vec;
use simples_pb;
use self::rocksdb::{RocksDB, RocksDBResult};
use rustc_serialize::json;

use tx;
use tx::Transaction;

#[derive(Default, Clone, Eq, PartialEq, Show, RustcEncodable, RustcDecodable)]
struct Balance {
    address: Vec<u8>,
    tokens: u64,
    op_index: u32
}

trait BalanceStore {
    fn get_balance(&self, address: &[u8]) -> Result<Balance, &str>;
    fn set_balance(&mut self, balance: &Balance) -> Result<(), &str>;
}

pub struct Store {
    db: RocksDB,
}

impl Drop for Store {
    fn drop(&mut self) {
        self.db.close();
    }
}

impl BalanceStore for Store {
    fn get_balance(&self, address: &[u8]) -> Result<Balance, &str> {
        let get_result = self.db.get(address);
        match get_result {
            RocksDBResult::Some(bytes) => {
                let decoded:Balance = json::decode(
                    bytes.to_utf8().unwrap()).unwrap();
                Ok(decoded)
            },
            RocksDBResult::None => Ok(Balance {
                    address: vec::as_vec(address).clone(),
                    tokens: 100u64,
                    op_index: 0u32
                }),
            RocksDBResult::Error(msg) => Err(msg)
        }
    }

    fn set_balance(&mut self, balance: &Balance) -> Result<(), &str> {
        let encoded = json::encode(&balance);
        let put_result = self.db.put(&balance.address[0..],
                                     encoded.unwrap().as_bytes());
        match put_result {
            Ok(()) => Ok(()),
            Err(msg) => Err(msg)
        }
    }
}

impl Store {
    pub fn new(db_path: &str) -> Self {
        Store {
            db: RocksDB::open_default(db_path).unwrap()
        }
    }

    pub fn mutate(&self) -> TransactionCache {
        TransactionCache::new(self)
    }
}

struct TransactionCache<'a> {
    cache: HashMap<Vec<u8>, Balance>,
    store: &'a (BalanceStore + 'a)
}

impl<'a> TransactionCache<'a> {
    fn new(store: &'a (BalanceStore + 'a)) -> TransactionCache<'a> {
        TransactionCache {
            cache: HashMap::<Vec<u8>, Balance>::new(),
            store: store
        }
    }

    fn get_balance(&mut self, address: &[u8]) -> Result<Balance, &'static str> {
        match self.cache.entry(vec::as_vec(address).clone()) {
            hash_map::Entry::Occupied(balance) => Ok(balance.get().clone()),
            hash_map::Entry::Vacant(balance) => {
                match self.store.get_balance(address) {
                   Ok(db_balance) => Ok(balance.insert(db_balance).clone()),
                   Err(msg) => Err("") // Fix: return err from RocksDB
                }
            }
        }
    }

    fn set_balance(&mut self, balance: &Balance) {
        self.cache.insert(balance.address.clone(), balance.clone());
    }

    pub fn clear(&mut self) {
        self.clear();
    }

    pub fn add_transfer(&mut self, transfer: &simples_pb::Transfer) ->
        Result<(), &'static str> {
        let mut src_balance =
                self.get_balance(transfer.get_source_pk()).unwrap();
        let mut dest_balance =
                self.get_balance(transfer.get_destination_pk()).unwrap();

        if src_balance.tokens >= transfer.get_tokens() {
            if transfer.get_op_index() == src_balance.op_index {
                println!("{}: {} -> {}", src_balance.op_index,
                         src_balance.tokens,
                         src_balance.tokens - transfer.get_tokens());

                src_balance.tokens -= transfer.get_tokens();
                dest_balance.tokens += transfer.get_tokens();
                src_balance.op_index += 1;
                self.set_balance(&src_balance);
                self.set_balance(&dest_balance);
                Ok(())
            } else {Err("Wrong op number.")}
        } else { Err("Not enough funds.") }
    }

    pub fn add_transaction(&mut self, transaction: &simples_pb::Transaction) ->
        Result<(), &'static str> {
        try!(transaction.check_signatures());
        for transfer in transaction.get_commit().get_transfers().iter() {
            match self.add_transfer(transfer) {
                Ok(_) => (),
                Err(msg) => return Err(msg)
            };
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), &str> {
        self.clear();
        Ok(())
    }
}
