use std::collections::hash_map::{self, HashMap};
use std::error::{self, FromError};
use std::vec;

use rustc_serialize::json;

use error::{SimplesError, SimplesResult};
use tx::{self, Transaction};
use store::{KeyValueStore, RocksStore};
use simples_pb;

#[derive(Default, Clone, Eq, PartialEq, Show, RustcEncodable, RustcDecodable)]
struct Balance {
    address: Vec<u8>,
    tokens: u64,
    op_index: u32
}

pub struct BalanceStore<Store: KeyValueStore> {
    kv_store: Store,
}

// TODO: Better name (not use Trait, how should the impl be called?)
pub trait BalanceStoreTrait {
    fn get_balance(&self, address: &[u8]) -> SimplesResult<Balance>;
    fn set_balance(&mut self, balance: &Balance) -> SimplesResult<()>;
}

impl<Store: KeyValueStore> BalanceStore<Store> {
    pub fn new(kv_store: Store) -> Self {
        BalanceStore { kv_store: kv_store }
    }

    pub fn mutate(&mut self) -> TransactionCache<Self> {
        TransactionCache::new(self)
    }
}

impl<Store: KeyValueStore> BalanceStoreTrait for BalanceStore<Store> {
    fn get_balance(&self, address: &[u8]) -> SimplesResult<Balance> {
        let get_result = try!(self.kv_store.get_bytes(address));
        match get_result {
            Some(bytes) => {
                let as_utf8 = ::std::str::from_utf8(&bytes[]).unwrap();
                let decoded:Balance = json::decode(as_utf8).unwrap();
                Ok(decoded)
            },
            None => Ok(Balance {
                address: address.to_vec(),
                tokens: 100u64,
                op_index: 0u32
            })
        }
    }

    fn set_balance(&mut self, balance: &Balance) -> SimplesResult<()> {
        let encoded = try!(json::encode(&balance));
        Ok(try!(self.kv_store.set_bytes(
            &balance.address[0..], encoded.as_bytes())))
    }
}

struct TransactionCache<'a, Store: 'a + BalanceStoreTrait> {
    cache: HashMap<Vec<u8>, Balance>,
    store: &'a mut Store
}

impl<'a, Store: 'a + BalanceStoreTrait> TransactionCache<'a, Store> {
    fn new(store: &'a mut Store) -> TransactionCache<'a, Store> {
        TransactionCache {
            cache: HashMap::<Vec<u8>, Balance>::new(),
            store: store
        }
    }

    fn get_balance(&mut self, address: &[u8]) -> SimplesResult<Balance> {
        match self.cache.entry(vec::as_vec(address).clone()) {
            hash_map::Entry::Occupied(balance) => Ok(balance.get().clone()),
            hash_map::Entry::Vacant(balance) => {
                let db_balance = try!(self.store.get_balance(address));
                Ok(balance.insert(db_balance).clone())
            }
        }
    }

    fn set_balance(&mut self, balance: &Balance) {
        self.cache.insert(balance.address.clone(), balance.clone());
    }

    pub fn clear(&mut self) { self.cache.clear(); }

    pub fn add_transfer(&mut self, transfer: &simples_pb::Transfer) ->
        SimplesResult<()> {
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
            } else { Err(FromError::from_error("Wrong op number.")) }
        } else { Err(FromError::from_error("Not enough funds.")) }
    }

    pub fn apply_transaction(
        &mut self, transaction: &simples_pb::Transaction) -> SimplesResult<()>
    {
        try!(transaction.check_signatures());
        for transfer in transaction.get_commit().get_transfers().iter() {
            try!(self.add_transfer(transfer));
        }
        Ok(())
    }

    pub fn flush(&mut self) -> SimplesResult<()> {
        for (address, balance) in self.cache.iter() {
            try!(self.store.set_balance(balance));
        }
        self.clear();
        Ok(())
    }
}
