use std::cmp::min;
use std::error::Error;
use std::io::{Read, Write};
use std::iter::FromIterator;

use protobuf::{self, Message, RepeatedField};
use time::now_utc;
use uuid::Uuid;

use block::{HashedBlockExt, SignedBlockExt};
use blocktree::BlockTreeStore;
use crypto::HashDigest;
use error::{SimplesError, SimplesResult};
use nanomsg::{Endpoint, Protocol, Socket};
use simples_pb::{self, HashedBlock, GetBlocksRequest,
                 GetBlocksResponse, GetBlocktreeRequest,
                 GetBlocktreeResponse, PubBlockRequest,
                 PubBlockResponse, PubTransactionRequest,
                 PubTransactionResponse, RpcRequest, RpcResponse,
                 RpcRequest_Method, SignedRpcRequest, Transaction};
use staking::BlockTemplate;
use store::RocksStore;
use tx::TransactionExt;

pub fn wrap_get_blocks_request(request: GetBlocksRequest) -> RpcRequest {
    let mut wrapped_request = RpcRequest::new();
    wrapped_request.set_method(RpcRequest_Method::GET_BLOCKS);
    wrapped_request.set_get_blocks(request);
    wrapped_request
}

pub fn wrap_get_blocktree_request(request: GetBlocktreeRequest) -> RpcRequest {
    let mut wrapped_request = RpcRequest::new();
    wrapped_request.set_method(RpcRequest_Method::GET_BLOCKTREE);
    wrapped_request.set_get_blocktree(request);
    wrapped_request
}

pub fn wrap_pub_block_request(request: PubBlockRequest) -> RpcRequest {
    let mut wrapped_request = RpcRequest::new();
    wrapped_request.set_method(RpcRequest_Method::PUB_BLOCK);
    wrapped_request.set_pub_block(request);
    wrapped_request
}

pub fn wrap_pub_transaction_request(request: PubTransactionRequest) -> RpcRequest {
    let mut wrapped_request = RpcRequest::new();
    wrapped_request.set_method(RpcRequest_Method::PUB_TRANSACTION);
    wrapped_request.set_pub_transaction(request);
    wrapped_request
}

pub trait RpcService {
    fn get_blocks(&mut self, request: GetBlocksRequest) ->
        SimplesResult<GetBlocksResponse>;
    fn get_blocktree(&mut self, request: GetBlocktreeRequest) ->
        SimplesResult<GetBlocktreeResponse>;
    fn pub_block(&mut self, request: PubBlockRequest) ->
        SimplesResult<PubBlockResponse>;
    fn pub_transaction(&mut self, request: PubTransactionRequest) ->
        SimplesResult<PubTransactionResponse>;
}

pub trait StakerService {
    fn on_successful_stake(&mut self, template: BlockTemplate)
                           -> SimplesResult<()>;
}

pub trait HeadBlockPubService {
    fn get_pub_endpoint(&self) -> &str;
    fn current_head_block(&self) -> SimplesResult<HashedBlock>;
}

pub trait SyncBlocktree {
    fn on_peer_blocktree_update(&mut self, response: GetBlocktreeResponse)
                                -> SimplesResult<Option<RpcRequest>>;
    fn on_received_peer_blocks(&mut self, response: GetBlocksResponse)
                               -> SimplesResult<Option<RpcRequest>>;
}

pub struct SimplesService {
    blocktree: BlockTreeStore<RocksStore>,
    pending_transactions: Vec<Transaction>,
    pub_block_socket: Socket,
    pub_block_endpoint: Endpoint,
    pub_block_endpoint_str: String
}

impl SimplesService {
    pub fn new(blocktree: BlockTreeStore<RocksStore>)
               -> SimplesResult<SimplesService> {
        println!("Head block: {}", blocktree.get_head_hash().unwrap());

        let pub_endpoint_str = format!("inproc://{}", Uuid::new_v4().to_string());
        let mut pub_socket = try!(Socket::new(Protocol::Pub));
        let pub_endpoint = try!(pub_socket.bind(&pub_endpoint_str));
        Ok(SimplesService {
            blocktree: blocktree,
            pending_transactions: vec![],
            pub_block_socket: pub_socket,
            pub_block_endpoint: pub_endpoint,
            pub_block_endpoint_str: pub_endpoint_str
        })
    }

    fn prune_invalid_transactions(&mut self) {
        let mut snapshot = self.blocktree.snapshot();
        let pending = self.pending_transactions.drain().filter(
            |tx| snapshot.apply_transaction(&tx).is_ok()).collect();
        self.pending_transactions = pending;
    }

    fn publish_new_head(&mut self, head: &HashedBlock) -> SimplesResult<()> {
        let head_bytes = &try!(head.write_to_bytes());
        try!(self.pub_block_socket.nb_write(&head_bytes));

        Ok(())
    }

    fn set_head(&mut self, head: &HashedBlock) -> SimplesResult<()> {
        let head_hash = try!(head.decode_hash());
        let previous_hash = try!(head.decode_previous());
        try!(self.blocktree.set_head(&head_hash));
        self.prune_invalid_transactions();
        println!("======  New head block  ======
   Height: {}
     Head: {}
 Previous: {}
Staker PK: {}
Timestamp: {}
    Proof: {}
      #tx: {}",
                 head.get_height(), head_hash, previous_hash,
                 try!(head.decode_staker_pk()),
                 head.get_block().get_timestamp(),
                 try!(head.decode_proof()),
                 head.get_block().get_transactions().len());
        self.publish_new_head(head)
    }
}

impl StakerService for SimplesService {
    fn on_successful_stake(&mut self, template: BlockTemplate)
                           -> SimplesResult<()>
    {
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
            block.set_staker_pk(template.staker_pk.0.to_vec());
            block.set_previous(template.previous_block.0.to_vec());
            block.set_timestamp(template.timestamp);
            block.set_height(block_height);
            block.set_target_hash(template.proof_hash.0.to_vec());

            self.prune_invalid_transactions();
            block.set_transactions(
                FromIterator::from_iter(self.pending_transactions.drain()));
        }
        staked_block.mut_signed_block().sign(&template.staker_sk);
        let staked_hash = staked_block.compute_hash();
        try!(self.blocktree.insert_block(staked_block.clone()));
        try!(self.set_head(&staked_block));

        // println!("\n============");
        // println!("Sucessful stake ({} tx) now={} head_time={} height={}:\n\
        //           new head: {}\nprev_head: {}",num_tx, now_utc().to_timespec().sec,
        //          template.timestamp, block_height, staked_hash,
        //          template.previous_block);
        // println!("============\n");

        Ok(())
    }
}

impl HeadBlockPubService for SimplesService {
    fn get_pub_endpoint(&self) -> &str { &self.pub_block_endpoint_str }

    fn current_head_block(&self) -> SimplesResult<HashedBlock> {
        self.blocktree.get_head()
    }
}

impl RpcService for SimplesService {
    fn get_blocks(&mut self, mut request: GetBlocksRequest) ->
        SimplesResult<GetBlocksResponse>
    {
        use simples_pb::GetBlocksResponse_Status as ResponseStatus;
        let mut response = GetBlocksResponse::new();
        response.set_status(ResponseStatus::OK);
        let mut blocks = vec![];
        for hash_bytes in request.take_blocks().into_iter() {
            let block_hash = match HashDigest::from_slice(&hash_bytes) {
                Ok(h) => h,
                Err(err) => {
                    response.set_status(ResponseStatus::INVALID_HASH);
                    response.set_description(format!("{}", err));
                    return Ok(response);
                }
            };
            let block = match try!(self.blocktree.get_block(&block_hash)) {
                Some(b) => b,
                None => {
                    response.set_status(ResponseStatus::UNKNOWN_BLOCK);
                    response.set_description(format!(
                        "Unknown block with hash {}", block_hash));
                    return Ok(response);
                }
            };
            blocks.push(block);
        }
        response.set_blocks(RepeatedField::from_vec(blocks));
        Ok(response)
    }

    fn get_blocktree(&mut self, request: GetBlocktreeRequest) ->
        SimplesResult<GetBlocktreeResponse>
    {
        use simples_pb::GetBlocktreeResponse_Status as ResponseStatus;
        let mut response = GetBlocktreeResponse::new();
        let mut block_pointer = try!(self.blocktree.get_head());
        response.set_status(ResponseStatus::OK);
        response.set_head(block_pointer.get_hash().to_vec());
        response.set_head_height(block_pointer.get_height());
        response.set_start_height(
            min(request.get_start_height(), block_pointer.get_height()));

        if request.get_start_height() > block_pointer.get_height() {
            response.set_ancestors(RepeatedField::from_vec(vec![]));
            return Ok(response);
        }
        let mut ancestors = vec![block_pointer.get_hash().to_vec()];
        while block_pointer.get_height() > request.get_start_height() {
            block_pointer = try!(self.blocktree.get_block(
                &try!(block_pointer.decode_previous()))).unwrap();
            ancestors.push(block_pointer.get_hash().to_vec());
        }
        // println!("ancestors: {:?}", ancestors);
        ancestors.reverse();
        response.set_ancestors(RepeatedField::from_vec(ancestors));
        Ok(response)
    }

    fn pub_block(&mut self, request: PubBlockRequest) ->
        SimplesResult<PubBlockResponse> {
        use simples_pb::PubBlockResponse_Status as ResponseStatus;
        println!("Got a pub_block request!");

        let mut response = PubBlockResponse::new();
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
        let block_hash: HashDigest =
            HashDigest::from_slice(hashed_block.get_hash()).unwrap();
        if block_hash == try!(self.blocktree.get_head_hash()) {
            println!("Head is already there!");
            response.set_status(ResponseStatus::OK);
            return Ok(response);
        }

        let maybe_prev_head: SimplesResult<HashDigest> =
                HashDigest::from_slice(hashed_block.get_block().get_previous());
        if maybe_prev_head.is_err() {
            println!("Received a block with invalid previous from peer");
            response.set_status(ResponseStatus::INVALID_BLOCK);
            return Ok(response);
        }
        response.set_status(ResponseStatus::OK);
        Ok(response)
    }

    fn pub_transaction(&mut self, mut request: PubTransactionRequest)
                       -> SimplesResult<PubTransactionResponse> {
        use simples_pb::PubTransactionResponse_Status as ResponseStatus;

        let mut response = PubTransactionResponse::new();
        if !request.has_transaction() {
            println!("Missing trans {:?}", request.has_transaction());
            response.set_status(ResponseStatus::INVALID_REQUEST);
            return Ok(response);
        }
        let transaction = request.take_transaction();
        let checked = transaction.verify_signatures();
        if checked.is_err() {
            response.set_status(ResponseStatus::INVALID_REQUEST);
            response.set_description(
                String::from_str(checked.unwrap_err().description()));
            return Ok(response);
        }

        self.prune_invalid_transactions();
        let mut snapshot = self.blocktree.snapshot();
        for tx in self.pending_transactions.iter() {
            let apply_result = snapshot.apply_transaction(tx);
            assert!(apply_result.is_ok());
        }

        let applied = snapshot.apply_transaction(&transaction);
        if applied.is_err() {
            response.set_status(ResponseStatus::INVALID_REQUEST);
            response.set_description(
                String::from_str(applied.unwrap_err().description()));
            return Ok(response);
        }
        self.pending_transactions.push(transaction);
        response.set_status(ResponseStatus::OK);
        Ok(response)
    }
}

impl SyncBlocktree for SimplesService {
    fn on_peer_blocktree_update(&mut self, mut response: GetBlocktreeResponse)
                                -> SimplesResult<Option<RpcRequest>>
    {
        let head = try!(self.blocktree.get_head());
        println!("Got blocktree state from peer with heights={}..{} / \
                  current head height: {}", response.get_start_height(),
                 response.get_start_height() + response.get_ancestors().len() as u32,
                 head.get_height());
        if head.get_height() >= response.get_head_height() ||
            response.get_ancestors().len() == 0 {
            return Ok(None)
        }
        let mut missing_blocks = vec![];
        let mut oldest_block = true;
        for hash_bytes in response.take_ancestors().into_iter() {
            let maybe_ancestor_hash = HashDigest::from_slice(&hash_bytes);
            if maybe_ancestor_hash.is_err() {
                println!("Received GetBlocktreeResponse containing an invalid \
                          block hash.");
                return Ok(None)
            }
            let ancestor_hash = maybe_ancestor_hash.unwrap();
            match try!(self.blocktree.get_block(&ancestor_hash)) {
                Some(_) => {},
                None => {
                    println!("requesting block: {}", ancestor_hash);
                    if oldest_block {
                        if response.get_start_height() == 0 {
                            println!("WARNING: peer uses a different genesis \
                                      block: {}", ancestor_hash);
                            return Ok(None)
                        }
                        let mut new_start_height = response.get_start_height();
                        if new_start_height < 20 {
                            new_start_height = 0;
                        } else {
                            new_start_height -= 20;
                        }
                        let mut request = GetBlocktreeRequest::new();
                        request.set_start_height(new_start_height);
                        return Ok(Some(wrap_get_blocktree_request(request)));
                    } else {
                        missing_blocks.push(hash_bytes);
                    }
                }
            }
            oldest_block = false;
        }
        let mut request = GetBlocksRequest::new();
        request.set_blocks(RepeatedField::from_vec(missing_blocks));
        return Ok(Some(wrap_get_blocks_request(request)));
    }

    fn on_received_peer_blocks(&mut self, mut response: GetBlocksResponse)
                               -> SimplesResult<Option<RpcRequest>>
    {
        println!("Received {} blocks from peer.", response.get_blocks().len());
        let mut new_head = try!(self.blocktree.get_head());
        for block in response.take_blocks().into_iter() {
            if block.get_height() > new_head.get_height() {
                new_head = block.clone();
            }
            try!(self.blocktree.insert_block(block));
        }
        try!(self.blocktree.set_head(&try!(new_head.decode_hash())));
        Ok(None)
    }
}

pub struct Client {
    pub endpoint_str: String,
    endpoint: Endpoint,
    socket: Socket
}

impl Client {
    pub fn new(endpoint_str: &str) -> SimplesResult<Client> {
        let mut socket = try!(Socket::new(Protocol::Req));
        Ok(Client {
            endpoint_str: String::from_str(endpoint_str),
            endpoint: try!(socket.connect(endpoint_str)),
            socket: socket
        })
    }

    fn dispatch(&mut self, request: &SignedRpcRequest)
                -> SimplesResult<RpcResponse> {
        let request_bytes = try!(request.write_to_bytes());
        try!(self.socket.write_all(&request_bytes));

        let mut response_bytes = vec![];
        try!(self.socket.read_to_end(&mut response_bytes));
        Ok(try!(protobuf::parse_from_bytes(&response_bytes)))
    }
}

impl Drop for Client {
    fn drop(&mut self) { self.endpoint.shutdown().unwrap(); }
}

impl RpcService for Client {
    fn get_blocks(&mut self, request: GetBlocksRequest) ->
        SimplesResult<GetBlocksResponse> {
        let mut signed_request = SignedRpcRequest::new();
        signed_request.set_request(wrap_get_blocks_request(request));
        self.dispatch(&signed_request).map(
            |mut response| response.take_get_blocks())
    }

    fn get_blocktree(&mut self, request: GetBlocktreeRequest) ->
        SimplesResult<GetBlocktreeResponse> {
        let mut signed_request = SignedRpcRequest::new();
        signed_request.set_request(wrap_get_blocktree_request(request));
        self.dispatch(&signed_request).map(
            |mut response| response.take_get_blocktree())
    }

    fn pub_block(&mut self, request: PubBlockRequest) ->
        SimplesResult<PubBlockResponse> {
        let mut signed_request = SignedRpcRequest::new();
        signed_request.set_request(wrap_pub_block_request(request));
        self.dispatch(&signed_request).map(
            |mut response| response.take_pub_block())
    }

    fn pub_transaction(&mut self, request: PubTransactionRequest) ->
        SimplesResult<PubTransactionResponse> {
        let mut signed_request = SignedRpcRequest::new();
        signed_request.set_request(wrap_pub_transaction_request(request));
        self.dispatch(&signed_request).map(
            |mut response| response.take_pub_transaction())
    }
}
