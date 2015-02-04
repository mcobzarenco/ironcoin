use std::sync::mpsc::{channel, Sender, Receiver};

use protobuf::{self, Message};

use balance::BalanceStore;
use crypto;
use error::SimplesResult;
use simples_pb::{self, PublishTransactionRequest, PublishTransactionResponse,
                 PublishTransactionResponse_Status, HashedBlock};
use staking::{ThreadedStaker};
use store::{ProtoStore, RocksStore};
use tx::Transaction;
use block;

pub trait Service {
    fn pub_transaction(&mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>;
}

pub struct SimplesService {
    balance_store: BalanceStore<RocksStore>,
    block_store: ProtoStore<RocksStore>,
    staker: ThreadedStaker
}

impl Service for SimplesService {
    fn pub_transaction(&mut self, request: PublishTransactionRequest)
                       -> SimplesResult<PublishTransactionResponse>
    {
        let mut response = PublishTransactionResponse::new();
        if !request.has_transaction() {
            println!("Missing trans {:?}", request.has_transaction());
            response.set_status(PublishTransactionResponse_Status::INVALID_REQUEST);
            return Ok(response);
        }
        let transaction = request.get_transaction();
        // println!("{}", protobuf::text_format::print_to_string(&transaction));
        let checked = transaction.check_signatures();
        if checked.is_err() {
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
    pub fn new(balance_db: &str, block_db: &str)
               -> SimplesResult<SimplesService>
    {
        let balance_store = BalanceStore::new(try!(RocksStore::new(balance_db)));
        let block_store = ProtoStore::new(try!(RocksStore::new(block_db)));
        let staker = try!(ThreadedStaker::new(crypto::hash(b"a"), 100));

        Ok(SimplesService {
            balance_store: balance_store,
            block_store: block_store,
            staker: staker
        })
    }
}
