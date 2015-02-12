use protobuf::Message;
use time::now_utc;
use uuid::Uuid;

use balance::BalanceStore;
use block::{BlockStore, HashedBlockExt};
use crypto::HashDigest;
use error::SimplesResult;
use nanomsg::{Endpoint, Protocol, Socket};
use simples_pb::{self, HashedBlock,
                 PublishTransactionRequest, PublishTransactionResponse,
                 PublishBlockRequest, PublishBlockResponse, Transaction};
use staking::BlockTemplate;
use store::RocksStore;
use tx::TransactionExt;

pub trait RpcService {
    fn pub_block(&mut self, request: PublishBlockRequest) ->
        SimplesResult<PublishBlockResponse>;
    fn pub_transaction(&mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>;
}

pub trait StakerService {
    fn on_successful_stake(&mut self, template: BlockTemplate)
                           -> SimplesResult<()>;
}

pub trait HeadBlockPubService {
    fn get_pub_endpoint(&self) -> &str;
    fn current_head_block(&self) -> &HashedBlock;
}

pub struct SimplesService {
    balance_store: BalanceStore<RocksStore>,
    block_store: BlockStore<RocksStore>,
    pending_transactions: Vec<simples_pb::Transaction>,
    pub_block_socket: Socket,
    pub_block_endpoint: Endpoint,
    pub_block_endpoint_str: String
}

impl StakerService for SimplesService {
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
        try!(self.block_store.set_head(hashed_block.clone()));

        {
            let mut cache = self.balance_store.mutate();
            for tx in self.pending_transactions.iter() {
                let apply_result = cache.apply_transaction(tx);
                assert!(apply_result.is_ok());
            }
            try!(cache.flush());
        }
        // try!(self.staker_head.as_mut().unwrap().set_head_block(
        //     head_hash.clone(), prev_head_hash.clone(), now_utc().to_timespec().sec));
        try!(self.publish_block(hashed_block.clone()));
        println!("\n============");
        println!("Sucessful stake ({} tx) now={} head_time={}:\nnew head: {}\nprev_head: {}",
                 self.pending_transactions.len(), now_utc().to_timespec().sec,
                 template.timestamp, head_hash, prev_head_hash);
        println!("============\n");
        self.pending_transactions.clear();

        // for peer in range(0, self.peers.len()) {
        //     let mut request = PublishBlockRequest::new();
        //     request.set_block(hashed_block.clone());
        //     let response = self.peers[peer].pub_block(request);
        //     if response.is_err() {
        //         println!("Could not pub_block to peer \"{}\":P{}",
        //                  self.peer_endpoints[peer], response.as_ref().err().unwrap());
        //     }
        //     println!("pub_block: got status {:?}", response.unwrap().get_status());
        // }
        Ok(())
    }

}

impl HeadBlockPubService for SimplesService {
    fn get_pub_endpoint(&self) -> &str { &self.pub_block_endpoint_str[] }

    fn current_head_block(&self) -> &HashedBlock { self.block_store.get_head() }
}

impl RpcService for SimplesService {
    fn pub_block(&mut self, request: PublishBlockRequest) ->
        SimplesResult<PublishBlockResponse>
    {
        use simples_pb::PublishBlockResponse_Status as ResponseStatus;

        println!("Got a block: {:?}", request);

        let mut response = PublishBlockResponse::new();
        if !request.has_block() {
            println!("Missing block {:?}", request.has_block());
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }
        let hashed_block = request.get_block();
        if hashed_block.verify_hash().is_err() {
            println!("Received a block with invalid hash from peer");
            response.set_status(ResponseStatus::INVALID_BLOCK);
            return Ok(response);
        }
        println!("Block is valid;");
        response.set_status(ResponseStatus::OK);
        Ok(response)
    }

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
                    genesis.mut_signed_block().mut_block().set_timestamp(
                        now_utc().to_timespec().sec);
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

        let pub_endpoint_str = format!("inproc://{}", Uuid::new_v4().to_string());
        let mut pub_socket = try!(Socket::new(Protocol::Pub));
        let pub_endpoint = try!(pub_socket.bind(&pub_endpoint_str[]));

        Ok(SimplesService {
            balance_store: balance_store,
            block_store: block_store,
            pending_transactions: vec![],
            pub_block_socket: pub_socket,
            pub_block_endpoint: pub_endpoint,
            pub_block_endpoint_str: pub_endpoint_str
        })
    }

    fn publish_block(&mut self, block: HashedBlock) -> SimplesResult<()> {
        let block_bytes = &try!(block.write_to_bytes())[];
        try!(self.pub_block_socket.write_all(block_bytes));
        Ok(())
    }
}
