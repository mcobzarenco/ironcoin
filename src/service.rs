use time::now_utc;

use balance::BalanceStore;
use block::{BlockStore, HashedBlockExt};
use crypto::{HashDigest, PublicKey, SecretKey};
use error::SimplesResult;
use simples_pb::{self, PublishTransactionRequest, PublishTransactionResponse,
                 HashedBlock};
use staking::{BlockTemplate, HeadBlockUpdater};
use store::RocksStore;
use tx::Transaction;

pub trait Service {
    fn pub_transaction(&mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>;
}

pub trait HasStaker {
    fn on_successful_stake(&mut self, template: BlockTemplate)
                           -> SimplesResult<()>;
    fn update_staker(&mut self, staker_head: HeadBlockUpdater)
                     -> SimplesResult<()> ;
}

pub struct SimplesService {
    balance_store: BalanceStore<RocksStore>,
    block_store: BlockStore<RocksStore>,
    staker_head: Option<HeadBlockUpdater>,
    pending_transactions: Vec<simples_pb::Transaction>,
    staking_keys: Vec<(SecretKey, PublicKey)>
}

impl HasStaker for SimplesService {
    fn on_successful_stake(&mut self, template: BlockTemplate)
                           -> SimplesResult<()>
    {
        let prev_head_hash = self.block_store.get_head_hash();
        if template.previous_block != prev_head_hash {
            println!("Ignoring block staked with prev: {} != head: {}.",
                     template.previous_block, prev_head_hash);
            return Ok(())
        }
        let mut hashed_block = HashedBlock::new();
        {
            let mut block = hashed_block.mut_signed_block().mut_block();
            block.set_timestamp(template.timestamp);
            block.set_previous(prev_head_hash.0.to_vec());
        }
        let head_hash = hashed_block.compute_hash();
        try!(self.block_store.set_head(hashed_block));

        let mut cache = self.balance_store.mutate();
        for tx in self.pending_transactions.iter() {
            let apply_result = cache.apply_transaction(tx);
            assert!(apply_result.is_ok());
        }
        try!(cache.flush());
        try!(self.staker_head.as_mut().unwrap().set_head_block(
            head_hash.clone(), prev_head_hash.clone(), now_utc().to_timespec().sec));
        println!("\n============");
        println!("Sucessful stake ({} tx) now={} head_time={}:\nnew head: {}\nprev_head: {}",
                 self.pending_transactions.len(), now_utc().to_timespec().sec,
                 template.timestamp, head_hash, prev_head_hash);
        println!("============\n");
        self.pending_transactions.clear();
        Ok(())
    }

    fn update_staker(&mut self, mut staker_head: HeadBlockUpdater)
                     -> SimplesResult<()> {
        let head_hash = self.block_store.get_head_hash();
        try!(staker_head.set_head_block(
            head_hash.clone(), head_hash, now_utc().to_timespec().sec));
        self.staker_head = Some(staker_head);
        Ok(())
    }
}


impl Service for SimplesService {
    fn pub_transaction(&mut self, mut request: PublishTransactionRequest)
                       -> SimplesResult<PublishTransactionResponse> {
        use simples_pb::PublishTransactionResponse_Status as ResponseStatus;

        let mut response = PublishTransactionResponse::new();
        if !request.has_transaction() {
            println!("Missing trans {:?}", request.has_transaction());
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }
        let transaction = request.take_transaction();
        let checked = transaction.check_signatures();
        if checked.is_err() {
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }

        let mut cache = self.balance_store.mutate();
        for tx in self.pending_transactions.iter() {
            let apply_result = cache.apply_transaction(tx);
            assert!(apply_result.is_ok());
        }

        let applied = cache.apply_transaction(&transaction);
        if applied.is_err() {
            println!("{:?}", applied);
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }
        self.pending_transactions.push(transaction);
        response.set_status(ResponseStatus::OK);
        Ok(response)
    }
}

impl SimplesService {
    pub fn new(balance_db: &str, block_db: &str)
               // staking_keys: &[(SecretKey, PublicKey)])
               -> SimplesResult<SimplesService>
    {
        let balance_store = BalanceStore::new(try!(RocksStore::new(balance_db)));
        let block_store =
            match BlockStore::new_from_existing(try!(RocksStore::new(block_db))) {
                Ok(store) => store,
                Err(_) => {
                    let mut genesis = HashedBlock::new();
                    genesis.mut_signed_block().mut_block().set_previous(
                        HashDigest::from_u64(0).0.to_vec());
                    genesis.compute_hash();
                    let genesis_hash
                        = HashDigest::from_bytes(genesis.get_hash()).unwrap();

                    println!("No genesis block, creating one: {}",
                             genesis_hash);
                    try!(BlockStore::new_with_genesis(
                        try!(RocksStore::new(block_db)), &genesis))
                }
            };
        println!("Head block: {:?}", block_store.get_head_hash());

        Ok(SimplesService {
            balance_store: balance_store,
            block_store: block_store,
            staker_head: None,
            pending_transactions: vec![],
            staking_keys: vec![]
        })
    }
}
