use std::sync::mpsc::{channel, Sender, Receiver};

use balance::{BalanceStore};
use error::{SimplesResult};
use simples_pb::{self, PublishTransactionRequest, PublishTransactionResponse,
                 PublishTransactionResponse_Status};
use staking::{ThreadedStaker};
use store;
use tx::Transaction;

pub trait Service {
    fn pub_transaction(&mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>;
}

pub struct SimplesService {
    balance_store: BalanceStore<store::RocksStore>,
    // staker: ThreadedStaker,
    tx_sender: Sender<simples_pb::Transaction>
}

impl Service for SimplesService {
    fn pub_transaction(
        &mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>
    {
        let mut response = PublishTransactionResponse::new();
        if !request.has_transaction() {
            println!("Missing trans {:?}", request.has_transaction());
            response.set_status(PublishTransactionResponse_Status::INVALID_REQUEST);
            return Ok(response);
        }
        println!("{:?}", request.has_transaction());
        let transaction = request.get_transaction();
        let checked = transaction.check_signatures();
        if checked.is_err() {
            println!("{:?}", checked);
            response.set_status(PublishTransactionResponse_Status::INVALID_REQUEST);
            return Ok(response);
        }

        let mut cache = self.balance_store.mutate();
        let applied = cache.apply_transaction(transaction);
        if applied.is_err() {
            println!("{:?}", applied);
            response.set_status(PublishTransactionResponse_Status::INVALID_REQUEST);
            return Ok(response);
        }
        cache.flush();

        response.set_status(PublishTransactionResponse_Status::OK);
        Ok(response)
            // tx_sender.send(msg).unwrap();
            // let reply = format!("reply");
    }
}

impl SimplesService {
    pub fn new(balance_db: &str) -> SimplesResult<SimplesService> {
        let rocks_store = try!(store::RocksStore::new(balance_db));
        let balance_store = BalanceStore::new(rocks_store);
        let (tx_sender, tx_receiver) = channel();

        Ok(SimplesService {
            balance_store: balance_store,
            // staker: staker,
            tx_sender: tx_sender
        })
    }
}
