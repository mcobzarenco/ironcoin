extern crate sodiumoxide;

use std::collections::HashMap;
use std::vec;

use sodiumoxide::crypto::hash::sha512;
use sodiumoxide::crypto::sign::ed25519::{
    self, PublicKey, SecretKey, PUBLICKEYBYTES, SECRETKEYBYTES,
    Signature, SIGNATUREBYTES, sign_detached, verify_detached};
use protobuf::Message;

use simples_pb;

pub fn slice_to_pk(bytes: &[u8]) -> Option<PublicKey> {
    if bytes.len() != PUBLICKEYBYTES { return None; }
    let mut key:[u8; PUBLICKEYBYTES] = [0; PUBLICKEYBYTES];
    for i in range(0, PUBLICKEYBYTES) {
        key[i] = bytes[i];
    }
    Some(PublicKey(key))
}

pub fn slice_to_sk(bytes: &[u8]) -> Option<SecretKey> {
    if bytes.len() != SECRETKEYBYTES { return None; }
    let mut key:[u8; SECRETKEYBYTES] = [0; SECRETKEYBYTES];
    for i in range(0, SECRETKEYBYTES) {
        key[i] = bytes[i];
    }
    Some(SecretKey(key))
}

pub fn slice_to_signature(bytes: &[u8]) -> Option<Signature> {
    if bytes.len() != SIGNATUREBYTES { return None; }
    let mut sign:[u8; SIGNATUREBYTES] = [0; SIGNATUREBYTES];
    for i in range(0, SIGNATUREBYTES) {
        sign[i] = bytes[i];
    }
    Some(Signature(sign))
}

pub trait Transaction {
    fn check_signatures(&self) -> Result<(), &'static str>;
}

impl Transaction for simples_pb::Transaction {
    fn check_signatures(&self) -> Result<(), &'static str> {
        let commit_bytes = &self.get_commit().write_to_bytes().unwrap()[];
        let mut sign_map = HashMap::<&[u8], &[u8]>::new();
        for sign in self.get_signatures().iter() {
            sign_map.insert(sign.get_public_key(), sign.get_payload());
        }
        for transfer in self.get_commit().get_transfers().iter() {
            match sign_map.get(transfer.get_source_pk()) {
                Some(sign_bytes) => {
                    let pk = slice_to_pk(transfer.get_source_pk()).unwrap();
                    let signature = slice_to_signature(&sign_bytes[]).unwrap();
                    if !verify_detached(&signature, commit_bytes, &pk) {
                        return Err("Invalid signature.")
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
        &mut self, sk: &SecretKey, source: &PublicKey, destination: &PublicKey,
        tokens: u64, op_index:u32) -> &mut Self {
        let mut transfer = simples_pb::Transfer::new();
        transfer.set_op_index(op_index);
        transfer.set_tokens(tokens);
        transfer.mut_source_pk().push_all(&source.0);
        transfer.mut_destination_pk().push_all(&destination.0);

        self.transfer_secret_keys.push(sk.clone());
        self.commit.mut_transfers().push(transfer);
        self
    }

    pub fn set_bounty(&mut self, sk: &SecretKey, source: &PublicKey,
                      bounty: u64) -> &mut Self {
        self.bounty_secret_key = Some(sk.clone());
        self.commit.mut_bounty_pk().push_all(&source.0);
        self.commit.set_bounty(bounty);
        self
    }

    pub fn build(self) -> Result<simples_pb::Transaction, &'static str> {
        let mut transaction = simples_pb::Transaction::new();
        let commit_bytes = &self.commit.write_to_bytes().unwrap()[];
        for (transfer, sk) in self.commit.get_transfers().iter()
            .zip(self.transfer_secret_keys.iter())
        {
            let signature = sign_detached(commit_bytes, sk);
            let pk_bytes = vec::as_vec(transfer.get_source_pk()).clone();
            let pk = slice_to_pk(&pk_bytes[]).unwrap();
            match verify_detached(&signature, commit_bytes, &pk) {
                true => {
                    let mut sign = simples_pb::DetachedSignature::new();
                    sign.set_public_key(pk_bytes);
                    sign.set_payload(signature.0.to_vec());
                    transaction.mut_signatures().push(sign);
                },
                false => return Err("Invalid key for source account.")
            }
        }
        transaction.set_commit(self.commit);
        try!(transaction.check_signatures());
        Ok(transaction)
    }
}
