extern crate sodiumoxide;
// extern crate "rustc-serialize" as rustc_serialize;

use std::collections::HashMap;
use std::vec;
use sodiumoxide::crypto::hash::sha512;
use sodiumoxide::crypto::sign::ed25519::{
    self, PublicKey, SecretKey, PUBLICKEYBYTES, SECRETKEYBYTES};
use protobuf::Message;
use simples_pb;

fn slice_to_pk(bytes: &[u8]) -> Option<PublicKey> {
    if bytes.len() != PUBLICKEYBYTES { return None; }
    let mut key:[u8; PUBLICKEYBYTES] = [0; PUBLICKEYBYTES];
    for i in range(0, PUBLICKEYBYTES) {
        key[i] = bytes[i];
    }
    Some(PublicKey(key))
}

fn hash_message<M: Message>(obj: &M) -> Vec<u8> {
    let bytes = &obj.write_to_bytes().unwrap()[];
    let sha512::Digest(commit_hash) = sha512::hash(&bytes[]);
    vec::as_vec(&commit_hash[]).clone()
}

pub trait Transaction {
    fn check_signatures(&self) -> Result<(), &'static str>;
}

impl Transaction for simples_pb::Transaction {
    fn check_signatures(&self) -> Result<(), &'static str> {
        let commit_hash = hash_message(self.get_commit());
        if commit_hash[] != self.get_commit_hash()[] {
            return Err("Invalid hash.");
        }
        let mut sign_map = HashMap::<&[u8], &[u8]>::new();
        for sign in self.get_signatures().iter() {
            sign_map.insert(sign.get_public_key(), sign.get_payload());
        }
        for transfer in self.get_commit().get_transfers().iter() {
            match sign_map.get(transfer.get_source_pk()) {
                Some(signature) => {
                    let pk = slice_to_pk(transfer.get_source_pk()).unwrap();
                    match ed25519::verify(&signature[], &pk) {
                        Some(hash) => {
                            if commit_hash[] != hash[] {
                                return Err("The incorrect hash is signed.")
                            }
                        },
                        None => return Err("Invalid signature.")
                    }
                },
                None => return Err("Missing key.")
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct TransactionBuilder {
    transfer_secret_keys: Vec<SecretKey>,
    bounty_secret_key: Option<SecretKey>,
    commit: simples_pb::Commitment
}

impl TransactionBuilder {
    pub fn new() -> TransactionBuilder {
        TransactionBuilder {
            transfer_secret_keys: Vec::<SecretKey>::new(),
            bounty_secret_key: None,
            commit: simples_pb::Commitment::new()
        }
    }

    pub fn add_transfer(
        mut self, sk: &SecretKey, source: &PublicKey, destination: &PublicKey,
        tokens: u64, op_index:u32) -> TransactionBuilder {
        let mut transfer = simples_pb::Transfer::new();
        transfer.set_op_index(op_index);
        transfer.set_tokens(tokens);
        transfer.mut_source_pk().push_all(&source.0);
        transfer.mut_destination_pk().push_all(&destination.0);

        self.transfer_secret_keys.push(sk.clone());
        self.commit.mut_transfers().push(transfer);
        self
    }

    pub fn set_bounty(mut self, sk: &SecretKey, source: &PublicKey,
                      bounty: u64) -> TransactionBuilder {
        self.bounty_secret_key = Some(sk.clone());
        self.commit.mut_bounty_pk().push_all(&source.0);
        self.commit.set_bounty(bounty);
        self
    }

    pub fn build(self) -> Result<simples_pb::Transaction, &'static str> {
        let commit_hash = hash_message(&self.commit);
        let mut transaction = simples_pb::Transaction::new();
        {
            let mut signatures = transaction.mut_signatures();
            let mut transfers_and_skeys = self.commit.get_transfers().iter()
                .zip(self.transfer_secret_keys.iter());

            for (transfer, sk) in transfers_and_skeys {
                let signature: Vec<u8> = ed25519::sign(&commit_hash[], sk);
                let pk_bytes = vec::as_vec(transfer.get_source_pk()).clone();
                let pk = slice_to_pk(&pk_bytes[]).unwrap();
                match ed25519::verify(&signature[0..], &pk) {
                    Some(_) => {
                        let mut sign = simples_pb::DetachedSignature::new();
                        sign.set_public_key(pk_bytes);
                        sign.set_payload(signature);
                        signatures.push(sign);
                    },
                    None => return Err("Invalid key for source account.")
                }
            }
        }
        transaction.set_commit(self.commit);
        transaction.set_commit_hash(commit_hash.clone());
        match transaction.check_signatures() {
            Ok(_) => Ok(transaction),
            Err(msg) => Err(msg)
        }
    }
}
