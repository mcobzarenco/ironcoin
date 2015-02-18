use std::collections::HashMap;
use std::vec;

use protobuf::Message;

use crypto::{PublicKey, SecretKey, sign, slice_to_signature, slice_to_pk,
             verify_signature};
use simples_pb::{Commitment, DetachedSignature, Transaction, Transfer};
use error::{SimplesError, SimplesResult};

pub trait TransactionExt {
    fn verify_signatures(&self) -> SimplesResult<()>;
}

impl TransactionExt for Transaction {
    fn verify_signatures(&self) -> SimplesResult<()> {
        let commit_bytes = &try!(self.get_commit().write_to_bytes())[];
        let mut sign_map = HashMap::<&[u8], &[u8]>::new();
        for sign in self.get_signatures().iter() {
            sign_map.insert(sign.get_public_key(), sign.get_payload());
        }
        for transfer in self.get_commit().get_transfers().iter() {
            match sign_map.get(transfer.get_source_pk()) {
                Some(sign_bytes) => {
                    let pk = try!(slice_to_pk(transfer.get_source_pk()));
                    let signature = try!(slice_to_signature(&sign_bytes[]));
                    if !verify_signature(&pk, commit_bytes, &signature) {
                        return Err(SimplesError::new("Invalid signature."))
                    }
                },
                None => return Err(SimplesError::new("Missing key."))
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct TransactionBuilder {
    transfer_secret_keys: Vec<SecretKey>,
    bounty_secret_key: Option<SecretKey>,
    commit: Commitment
}

impl TransactionBuilder {
    pub fn new() -> TransactionBuilder {
        TransactionBuilder {
            transfer_secret_keys: Vec::<SecretKey>::new(),
            bounty_secret_key: None,
            commit: Commitment::new()
        }
    }

    pub fn add_transfer(
        &mut self, sk: &SecretKey, source: &PublicKey, destination: &PublicKey,
        tokens: u64, op_index:u32) -> &mut Self {
        let mut transfer = Transfer::new();
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

    pub fn build(self) -> SimplesResult<Transaction> {
        let mut transaction = Transaction::new();
        let commit_bytes = &self.commit.write_to_bytes().unwrap()[];
        for (transfer, secret_key) in self.commit.get_transfers().iter()
            .zip(self.transfer_secret_keys.iter())
        {
            let signature = sign(secret_key, commit_bytes);
            let pk_bytes = vec::as_vec(transfer.get_source_pk()).clone();
            let pk = slice_to_pk(&pk_bytes[]).unwrap();
            match verify_signature(&pk, commit_bytes, &signature) {
                true => {
                    let mut sign = DetachedSignature::new();
                    sign.set_public_key(pk_bytes);
                    sign.set_payload(signature.0.to_vec());
                    transaction.mut_signatures().push(sign);
                },
                false => return Err(SimplesError::new("Invalid key for source account."))
            }
        }
        transaction.set_commit(self.commit);
        try!(transaction.verify_signatures());
        Ok(transaction)
    }
}
