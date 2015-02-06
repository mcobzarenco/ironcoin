use std::error::FromError;

use time::now_utc;

use balance::BalanceStore;
use block::{BlockStore, HashedBlockExt};
use crypto::{self, HashDigest};
use error::SimplesResult;
use simples_pb::{self, PublishTransactionRequest, PublishTransactionResponse,
                 HashedBlock};
use staking::{BlockTemplate, HeadBlockUpdater, ThreadedStaker};
use store::{ProtoStore, RocksStore};
use tx::Transaction;

pub trait Service {
    fn pub_transaction(&mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>;
}

pub trait HasStaker {
    fn on_successful_stake(&mut self, template: BlockTemplate);
    fn update_staker(&mut self, staker_head: HeadBlockUpdater);
}

pub struct SimplesService {
    balance_store: BalanceStore<RocksStore>,
    block_store: BlockStore<RocksStore>,
    staker_head: Option<HeadBlockUpdater>
}

impl HasStaker for SimplesService {
    fn on_successful_stake(&mut self, template: BlockTemplate) {
        println!("on_stake: {:?}", template);
    }

    fn update_staker(&mut self, staker_head: HeadBlockUpdater) {
        self.staker_head = Some(staker_head);
    }
}


impl Service for SimplesService {
    fn pub_transaction(&mut self, request: PublishTransactionRequest)
                       -> SimplesResult<PublishTransactionResponse> {
        use simples_pb::PublishTransactionResponse_Status as ResponseStatus;

        let mut response = PublishTransactionResponse::new();
        if !request.has_transaction() {
            println!("Missing trans {:?}", request.has_transaction());
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }
        let transaction = request.get_transaction();
        let checked = transaction.check_signatures();
        if checked.is_err() {
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }

        let mut cache = self.balance_store.mutate();
        let applied = cache.apply_transaction(transaction);
        if applied.is_err() {
            println!("{:?}", applied);
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }
        cache.flush();

        response.set_status(ResponseStatus::OK);
        Ok(response)
    }
}

const MSG_WRONG_GENESIS: &'static str =
    "Block DB at \"{}\" has genesis block: {} (request genesis block: {})";

impl SimplesService {
    pub fn new(balance_db: &str, block_db: &str) -> SimplesResult<SimplesService>
    {
        let balance_store = BalanceStore::new(try!(RocksStore::new(balance_db)));
        let mut block_store = BlockStore::new(try!(RocksStore::new(block_db)));
        let store_genesis = try!(block_store.get_genesis());
        let mut head_block;

        if store_genesis.is_none() {
            head_block = HashedBlock::new();
            head_block.mut_signed_block().mut_block().set_previous(
                HashDigest::from_u64(0).0.to_vec());
            head_block.compute_hash();

            println!("No genesis block, creating one: {}",
                     HashDigest::from_bytes(head_block.get_hash()).unwrap());
            try!(block_store.set_genesis(&head_block));
            try!(block_store.set_head(&head_block));
        } else {
            head_block = block_store.get_head().unwrap().unwrap();
        }
        println!("head_block: {:?}", head_block);
        // match (store_genesis, genesis) {
        //     (Some(genesis_block), Some(genesis_hash)) =>
        //         if genesis_block.get_hash() != genesis_hash {
        //             return Err(format!(MSG_WRONG_GENESIS, block_db,
        //                                store_genesis.unwrap(), genesis.unwrap())
        //                        );
        //         },

        // }

        let head_hash = HashDigest::from_bytes(head_block.get_hash()).unwrap();
        // let previous_hash = HashDigest::from_bytes(
        //     head_block.get_block().get_previous()).unwrap();
        let mut staker = try!(ThreadedStaker::new());
        try!(staker.set_head_block(
            head_hash.clone(), head_hash, now_utc().to_timespec().sec));
        Ok(SimplesService {
            balance_store: balance_store,
            block_store: block_store,
            staker_head: None
        })
    }
}
