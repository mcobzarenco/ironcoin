extern crate rocksdb;

use std::collections::hash_map::{self, HashMap};
use std::error::{FromError};
use std::ops::Deref;
use std::vec;

use protobuf::{self, MessageStatic};
use self::rocksdb::{RocksDB, RocksDBResult};

use error::{SimplesError, SimplesResult};

pub trait KeyValueStore {
    fn get_bytes(&self, key: &[u8]) -> SimplesResult<Option<Vec<u8>>>;
    fn set_bytes(&mut self, key: &[u8], value: &[u8]) -> SimplesResult<()>;
}

pub struct RocksStore {
    db: RocksDB,
}

impl RocksStore {
    pub fn new(db_path: &str) -> SimplesResult<Self> {
        let db = try!(RocksDB::open_default(db_path));
        Ok(RocksStore { db: db })
    }
}

impl Drop for RocksStore {
    fn drop(&mut self) {
        self.db.close();
    }
}

impl KeyValueStore for RocksStore {
    fn get_bytes(&self, key: &[u8]) -> SimplesResult<Option<Vec<u8>>> {
        match self.db.get(key) {
            RocksDBResult::Some(bytes) => // TODO: Remove copy.
                Ok(Some(vec::as_vec(bytes.deref()).clone())),
            RocksDBResult::None => Ok(None),
            RocksDBResult::Error(msg) => Err(FromError::from_error(msg))
        }
    }

    fn set_bytes(&mut self, key: &[u8], value: &[u8]) -> SimplesResult<()> {
        Ok(try!(self.db.put(key, value)))
    }
}

impl KeyValueStore for HashMap<Vec<u8>, Vec<u8>> {
    fn get_bytes(&self, key: &[u8]) -> SimplesResult<Option<Vec<u8>>> {
        Ok(match self.get(&key.to_vec()) {
            Some(bytes) => Some(bytes.clone()),
            None => None
        })
    }

    fn set_bytes(&mut self, key: &[u8], value: &[u8]) -> SimplesResult<()> {
        self.insert(key.to_vec(), value.to_vec());
        Ok(())
    }
}

pub trait ProtobufStore {
    fn get_message<Message: MessageStatic>(
        &self, key: &[u8]) -> SimplesResult<Option<Message>>;
    fn set_message<Message: MessageStatic>
        (&mut self, key: &[u8], message: &Message) -> SimplesResult<()>;
}

pub struct MessageStore<Store: KeyValueStore> {
    kv_store: Store
}

impl<Store: KeyValueStore> MessageStore<Store> {
    pub fn new(kv_store: Store) -> Self { MessageStore { kv_store: kv_store } }
}

impl<Store: KeyValueStore> ProtobufStore for MessageStore<Store> {
    fn get_message<Message: MessageStatic>
        (&self, key: &[u8]) -> SimplesResult<Option<Message>> {
        Ok(match try!(self.kv_store.get_bytes(key)) {
            Some(bytes) => Some(try!(protobuf::parse_from_bytes(&bytes[]))),
            None => None
        })
    }

    fn set_message<Message: MessageStatic>
        (&mut self, key: &[u8], message: &Message) -> SimplesResult<()>
    {
        let msg_bytes = &try!(message.write_to_bytes())[];
        Ok(try!(self.kv_store.set_bytes(key, msg_bytes)))
    }
}

impl<Store: KeyValueStore> KeyValueStore for MessageStore<Store> {
    fn get_bytes(&self, key: &[u8]) -> SimplesResult<Option<Vec<u8>>> {
        self.kv_store.get_bytes(key)
    }

    fn set_bytes(&mut self, key: &[u8], value: &[u8]) -> SimplesResult<()> {
        self.kv_store.set_bytes(key, value)
    }
}
