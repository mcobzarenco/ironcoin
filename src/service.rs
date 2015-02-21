use std::iter::FromIterator;

use protobuf::Message;
use time::now_utc;
use uuid::Uuid;

use block::HashedBlockExt;
use blocktree::BlockTreeStore;
use crypto::HashDigest;
use error::{SimplesError, SimplesResult};
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
    fn current_head_block(&self) -> SimplesResult<HashedBlock>;
}

pub struct SimplesService {
    blocktree: BlockTreeStore<RocksStore>,
    pending_transactions: Vec<simples_pb::Transaction>,
    pub_block_socket: Socket,
    pub_block_endpoint: Endpoint,
    pub_block_endpoint_str: String
}

impl StakerService for SimplesService {
    fn on_successful_stake(&mut self, template: BlockTemplate)
                           -> SimplesResult<()>
    {
        println!("Successfuly staked a new block!");
        let head_hash = try!(self.blocktree.get_head_hash());
        let previous_block = try!(try!(self.blocktree.get_block(
            &template.previous_block)).ok_or(SimplesError::new(&format!(
                "previous for staked block {} is missing from kv-store",
                template.previous_block))));
        if template.previous_block != head_hash {
            println!("WARNING: Staked block has previous {} != head: {}.",
                     template.previous_block, head_hash);
        }
        let mut staked_block = HashedBlock::new();
        let block_height = previous_block.get_block().get_height() + 1;
        let num_tx = self.pending_transactions.len();
        {
            let mut block = staked_block.mut_signed_block().mut_block();
            // block.set_staker_pk(template.staker_pk.to_vec());
            block.set_previous(template.previous_block.0.to_vec());
            block.set_timestamp(template.timestamp);
            block.set_height(block_height);
            block.set_target_hash(template.proof_hash.0.to_vec());

            block.set_transactions(
                FromIterator::from_iter(self.pending_transactions.drain()));
        }
        let staked_hash = staked_block.compute_hash();
        try!(self.blocktree.insert_block(staked_block.clone()));
        try!(self.blocktree.set_head(&staked_hash));
        try!(self.publish_block(staked_block.clone()));

        println!("\n============");
        println!(
            "Sucessful stake ({} tx) now={} head_time={} height={}:\nnew head: {}\nprev_head: {}",
            num_tx, now_utc().to_timespec().sec, template.timestamp, block_height,
            staked_hash, template.previous_block);
        println!("============\n");

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

    fn current_head_block(&self) -> SimplesResult<HashedBlock> {
        self.blocktree.get_head()
    }
}

impl RpcService for SimplesService {
    fn pub_block(&mut self, request: PublishBlockRequest) ->
        SimplesResult<PublishBlockResponse>
    {
        use simples_pb::PublishBlockResponse_Status as ResponseStatus;
        println!("Got a block!");

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
        let block_hash =
            HashDigest::from_bytes(hashed_block.get_hash()).unwrap();
        println!("Block is valid ({} tx), hash={}",
                 hashed_block.get_block().get_transactions().len(),
                 HashDigest::from_bytes(hashed_block.get_hash()).unwrap());
        if block_hash == try!(self.blocktree.get_head_hash()) {
            println!("Head is already there!");
            response.set_status(ResponseStatus::OK);
            return Ok(response);
        }

        let maybe_prev_head = HashDigest::from_bytes(
            hashed_block.get_block().get_previous());
        if maybe_prev_head.is_err() {
            println!("Received a block with invalid previous from peer");
            response.set_status(ResponseStatus::INVALID_BLOCK);
            return Ok(response);
        }
        if maybe_prev_head.unwrap() == try!(self.blocktree.get_head_hash()) {
            println!("Block comes after head. Trying to fast-forward.");
            {
                let mut snapshot = self.blocktree.snapshot();
                for tx in self.pending_transactions.iter() {
                    let apply_result = snapshot.apply_transaction(tx);
                    if apply_result.is_err() {
                        println!("Block contains invalid tx. Ignoring");
                        response.set_status(ResponseStatus::INVALID_BLOCK);
                        return Ok(response);
                    }
                }
                // try!(cache.flush());
            }
            // try!(self.blocktree.set_head(hashed_block.clone()));
            // try!(self.blocktree.set_head(hashed_block.));
            try!(self.publish_block(hashed_block.clone()));
        }
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
        let checked = transaction.verify_signatures();
        if checked.is_err() {
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }

        let mut snapshot = self.blocktree.snapshot();
        for tx in self.pending_transactions.iter() {
            let apply_result = snapshot.apply_transaction(tx);
            assert!(apply_result.is_ok());
        }

        let applied = snapshot.apply_transaction(&transaction);
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
    pub fn new(blocktree: BlockTreeStore<RocksStore>)
               -> SimplesResult<SimplesService>
    {
        println!("Head block: {:?}", blocktree.get_head_hash());

        let pub_endpoint_str = format!("inproc://{}", Uuid::new_v4().to_string());
        let mut pub_socket = try!(Socket::new(Protocol::Pub));
        let pub_endpoint = try!(pub_socket.bind(&pub_endpoint_str[]));
        Ok(SimplesService {
            blocktree: blocktree,
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
