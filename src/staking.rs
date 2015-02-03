use std::collections::HashSet;

use tx::{self, Transaction};
use sodiumoxide::crypto::hash::sha512::{self, Digest, HASHBYTES};

pub struct ThreadedStaker {
    tx_pool: HashSet<String>
}

// use std::sync::mpsc::{channel, Sender, Receiver};
// struct SimplesService {
//     tx_sender: Sender<simples_pb::Transaction>,
// }

// impl Service for SimplesService {
//     fn pub_transaction(
//         &mut self, request: simples_pb::PublishTransactionRequest) ->
//         SimplesResult<simples_pb::PublishTransactionResponse> {
//             println!("{:?}", request.get_transaction());
//             let response = simples_pb::PublishTransactionResponse::new();
//             Ok(response)
//             // tx_sender.send(msg).unwrap();
//             // let reply = format!("reply");
//         }
// }
