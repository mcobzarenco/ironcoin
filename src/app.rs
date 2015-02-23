use std::error::FromError;
use std::io::{Read, Write};
use std::time::Duration;

use nanomsg::{Endpoint, PollRequest, Protocol, Socket, NanoErrorKind};
use protobuf::{self, Message};

use block::HashedBlockExt;
use crypto::{gen_keypair, HashDigest, PublicKey, SecretKey, sign_message};
use error::{SimplesResult};
use service::{HeadBlockPubService, RpcService, StakerService, SyncBlocktree,
              wrap_get_blocks_request, wrap_get_blocktree_request,
              wrap_pub_block_request};
use simples_pb::{self, HashedBlock, GetBlocksRequest,
                 GetBlocktreeRequest, PubBlockRequest, RpcRequest,
                 RpcRequest_Method, RpcResponse, RpcResponse_Status,
                 SignedRpcRequest, Wallet};
use staking::{Staker, STAKING_INTERVAL};

pub struct Application<Service>
    where Service: RpcService + StakerService + HeadBlockPubService {
    app_public_key: PublicKey,
    app_secret_key: SecretKey,
    pub endpoint: String,
    peers: Vec<Peer>,
    service: Service,
    staker: Staker,
    sub_head_socket: Socket,
    sub_head_endpoint: Endpoint
}

impl<Service: HeadBlockPubService + RpcService + StakerService + SyncBlocktree>
    Application<Service> {
    pub fn new(endpoint: String, service: Service,
               peer_endpoints: Vec<String>, staking_keys: Wallet)
               -> SimplesResult<Application<Service>>
    {
        let mut sub_head_socket = try!(Socket::new(Protocol::Sub));
        try!(sub_head_socket.subscribe(""));
        let sub_head_endpoint = try!(
            sub_head_socket.connect(service.get_pub_endpoint()));
        let head = try!(service.current_head_block());
        let staker = Staker::new(staking_keys, try!(head.decode_hash()),
                                 head.get_block().get_timestamp());
        let mut peers = Vec::<Peer>::new();
        for peer_endpoint in peer_endpoints.into_iter() {
            peers.push(try!(Peer::new(peer_endpoint)));
        }
        println!("head block timestamp: {}", head.get_block().get_timestamp());

        let (pk, sk) = gen_keypair();
        Ok(Application {
            app_public_key: pk,
            app_secret_key: sk,
            endpoint: endpoint,
            peers: peers,
            service: service,
            staker: staker,
            sub_head_socket: sub_head_socket,
            sub_head_endpoint: sub_head_endpoint
        })
    }

    fn dispatch(&mut self, mut request: RpcRequest)
                    -> SimplesResult<RpcResponse>
    {
        let mut response = RpcResponse::new();
        match request.get_method() {
            RpcRequest_Method::INVALID => {
                println!("Peer tried to call invalid method.");
                response.set_status(RpcResponse_Status::INVALID_METHOD);
                Ok(response)
            },
            RpcRequest_Method::GET_BLOCKS => {
                response.set_status(RpcResponse_Status::OK);
                response.set_get_blocks(try!(
                    self.service.get_blocks(request.take_get_blocks())));
                Ok(response)
            },
            RpcRequest_Method::GET_BLOCKTREE => {
                response.set_status(RpcResponse_Status::OK);
                response.set_get_blocktree(try!(
                    self.service.get_blocktree(request.take_get_blocktree())));
                Ok(response)
            },
            RpcRequest_Method::PUB_BLOCK => {
                response.set_status(RpcResponse_Status::OK);
                response.set_pub_block(try!(
                    self.service.pub_block(request.take_pub_block())));
                Ok(response)
            },
            RpcRequest_Method::PUB_TRANSACTION => {
                response.set_status(RpcResponse_Status::OK);
                response.set_pub_transaction(try!(
                    self.service.pub_transaction(request.take_pub_transaction())));
                Ok(response)
            }
        }
    }

    fn handle_new_head_block(&mut self, new_head: HashedBlock)
                             -> SimplesResult<()> {
        println!("Publishing new head block to peers.");
        self.staker.set_head_block(try!(new_head.decode_hash()),
                                   new_head.get_block().get_timestamp());

        // for peer in self.peers.iter_mut() {
        //     let mut request = PubBlockRequest::new();
        //     request.set_block(new_head.clone());
        //     let send_result =
        //         peer.async_pub_block(request, Some(&self.app_secret_key));
        //     if send_result.is_err() {
        //         println!("WARNING: could not pub_block to peer \"{}\":P{}",
        //                  peer.endpoint, send_result.err().unwrap());
        //     }
        // }
        Ok(())
    }

    fn handle_peer_response(&mut self, peer_index: usize, mut response: RpcResponse)
                            -> SimplesResult<()>
    {
        let mut maybe_reply = None;
        println!("RESPONSE FOR METHOD: {:?}",
                 response.get_original_request().get_request().get_method());
        match response.get_original_request().get_request().get_method() {
            RpcRequest_Method::GET_BLOCKS => {
                println!("Peer replied to get_blocks");
                maybe_reply = try!(self.service.on_received_peer_blocks(
                    response.take_get_blocks()));
            },
            RpcRequest_Method::GET_BLOCKTREE => {
                println!("Peer replied to get_blocktree");
                maybe_reply = try!(self.service.on_peer_blocktree_update(
                    response.take_get_blocktree()));
            },
            RpcRequest_Method::PUB_BLOCK => {
                println!("Peer replied to pub_block");
            },
            _ => {
                println!("Peer sent unexpected reply.");
            }
        }
        println!("Replying to peer with {:?}", maybe_reply);
        match maybe_reply {
            Some(request) => {
                self.peers[peer_index].
                    sign_and_send(request, Some(&self.app_secret_key))
            },
            None => Ok(())
        }
    }

    fn handle_raw_rpc_request(&mut self, raw_request: &[u8])
                              -> SimplesResult<RpcResponse> {
        let parse_result: protobuf::ProtobufResult<SignedRpcRequest> =
            protobuf::parse_from_bytes(&raw_request);
        match parse_result {
            Ok(signed_request) => {
                let mut response =
                    try!(self.dispatch(signed_request.get_request().clone()));
                response.set_original_request(signed_request);
                Ok(response)
            }
            Err(err) => {
                println!("[app loop] Failed to parse protobuf '{:?}'.", err);
                let mut response = simples_pb::RpcResponse::new();
                response.set_status(
                    simples_pb::RpcResponse_Status::INVALID_MESSAGE);
                Ok(response)
            }
        }
    }

    fn handle_timeout(&mut self) -> SimplesResult<()> {
        self.try_staking_a_new_block()
    }

    fn probe_network_state(&mut self) -> SimplesResult<()> {
        let look_back = 10;
        let head_height = try!(self.service.current_head_block()).get_height();
        let start_height =
            if head_height > look_back {head_height - look_back} else { 0 };
        for peer_index in range(0, self.peers.len()) {
            let mut request = GetBlocktreeRequest::new();
            request.set_start_height(start_height);
            let send_result = self.peers[peer_index]
                .async_get_blocktree(request, Some(&self.app_secret_key));
            if send_result.is_err() {
                println!("WARNING: could call get_blocktree on peer \"{}\":P{}",
                         self.peers[peer_index].endpoint,
                         send_result.err().unwrap());
            }
        }
        Ok(())
    }

    fn try_staking_a_new_block(&mut self) -> SimplesResult<()> {
        match self.staker.stake_interval(STAKING_INTERVAL) {
            Some(template) => self.service.on_successful_stake(template),
            None => Ok(())
        }
    }

    pub fn run(&mut self) -> SimplesResult<()> {
        let mut rpc_socket = try!(Socket::new(Protocol::Rep));
        try!(rpc_socket.bind(&self.endpoint));
        let mut poll_fds = vec![rpc_socket.new_pollfd(true, false),
                                self.sub_head_socket.new_pollfd(true, false)];
        for peer in self.peers.iter() {
            poll_fds.push(peer.socket.new_pollfd(true, false));
        }
        let mut poll_request = PollRequest::new(&mut poll_fds);

        let mut probe = 0;
        println!("[app loop] RPC service is running at {}", self.endpoint);
        loop {
            let poll_result = Socket::poll(
                &mut poll_request, &Duration::seconds(STAKING_INTERVAL));
            if poll_result.is_err() {
                let err = poll_result.err().unwrap();
                if err.kind == NanoErrorKind::Timeout {
                    println!("Head height at timeout: {}",
                             self.service.current_head_block().unwrap().get_height());
                    try!(self.handle_timeout());
                    probe += 1;
                    if probe % 7 == 0 {
                        self.probe_network_state().unwrap()
                    }
                    continue;
                } else {
                    println!("[app loop] Failed to read in request '{}'. Exiting.",
                             err);
                    return Err(FromError::from_error(err));
                }
            }
            if poll_request.get_fds()[0].can_read() { // Rpc Request
                let mut request_bytes = vec![];
                match rpc_socket.read_to_end(&mut request_bytes) {
                    Ok(_) => {
                        let mut response;
                        let maybe_response =
                            self.handle_raw_rpc_request(&request_bytes);
                        if maybe_response.is_err() {
                            println!("Failed to handle rpc request err=\"{}\"",
                                     maybe_response.unwrap_err());
                            response = RpcResponse::new();
                            response.set_status(
                                simples_pb::RpcResponse_Status::INTERNAL_ERROR);
                        } else {
                            response = maybe_response.unwrap();
                        }
                        let response_bytes = response.write_to_bytes().unwrap();
                        let write_result = rpc_socket.write_all(&response_bytes);
                        if write_result.is_err() {
                            let err = write_result.err().unwrap();
                            println!("[app loop] Failed to send response '{}'. Exiting.",
                                     err);
                        }
                    },
                    Err(err) => {
                        println!("[app loop] Failed to read in request '{}'. \
                                  Exiting.", err);
                        return Err(FromError::from_error(err));
                    }
                }
            }
            if poll_request.get_fds()[1].can_read() { // New head block
                let mut msg_bytes = vec![];
                match self.sub_head_socket.read_to_end(&mut msg_bytes) {
                    Ok(_) => {
                        try!(self.handle_new_head_block(
                            try!(protobuf::parse_from_bytes(&msg_bytes))));
                    },
                    Err(err) => {
                        println!("[app loop] Failed to read in published new \
                                  block '{}'.", err);
                        return Err(FromError::from_error(err));
                    }
                };
            }
            for index in range(0, self.peers.len()) {
                if poll_request.get_fds()[index + 2].can_read() {
                    let recv_result =
                        self.peers[index].recv(Some(&self.app_public_key));
                    if recv_result.is_err() {
                        println!("[app loop] Failed to read in message from \
                                   peer {} with {}.", self.peers[index].endpoint,
                                 recv_result.unwrap_err());
                        break;
                    } else {
                        let response = recv_result.unwrap();
                        match self.handle_peer_response(index, response) {
                            Ok(_) => {},
                            Err(err) => {
                                println!("[app loop] Failed to handle peer response: \
                                          {} (for peer {}).", err,
                                         self.peers[index].endpoint);
                            }
                        }
                    }
                }
            }
            // for (index, peer) in self.peers.iter_mut().enumerate() {
            //     if poll_request.get_fds()[index + 2].can_read() {
            //         match peer.recv(Some(&self.app_public_key)) {
            //             Ok(response) => {
            //                 self.handle_peer_response(response);
            //             },
            //             Err(err) => {
            //                 println!("[app loop] Failed to read in message from \
            //                           peer {} with {}.", peer.endpoint, err);
            //             }
            //         }
            //     }
            // }
        } // loop
    }
}

#[unsafe_destructor]
impl<Service> Drop for Application<Service> where
    Service: HeadBlockPubService + RpcService + StakerService + SyncBlocktree {
    fn drop(&mut self) { self.sub_head_endpoint.shutdown().unwrap(); }
}

struct Peer {
    pub endpoint: String,
    pub socket: Socket
}

impl Peer {
    fn new(endpoint: String) -> SimplesResult<Peer> {
        let mut socket = try!(Socket::new(Protocol::Req));
        try!(socket.connect(&endpoint));
        Ok(Peer {
            endpoint: endpoint,
            socket: socket
        })
    }

    fn make_signed_request(request: RpcRequest, key: Option<&SecretKey>)
                           -> SignedRpcRequest {
        let mut signed_request = SignedRpcRequest::new();
        if key.is_some() {
            let signature = sign_message(key.unwrap(), &request);
            signed_request.set_signature(signature.0.to_vec());
        }
        signed_request.set_request(request);
        signed_request
    }

    fn sign_and_send(&mut self, request: RpcRequest, key: Option<&SecretKey>)
                     -> SimplesResult<()> {
        self.send(&Peer::make_signed_request(request, key))
    }

    fn send(&mut self, request: &SignedRpcRequest) -> SimplesResult<()> {
        let request_bytes = try!(request.write_to_bytes());
        try!(self.socket.nb_write(&request_bytes));
        Ok(())
    }

    fn recv(&mut self, check_sign: Option<&PublicKey>)
            -> SimplesResult<RpcResponse> {
        let mut response_bytes = vec![];
        try!(self.socket.read_to_end(&mut response_bytes));
        let response = try!(protobuf::parse_from_bytes(&response_bytes));
        Ok(response)
    }

    fn async_get_blocks(&mut self, request: GetBlocksRequest,
                        key: Option<&SecretKey>) -> SimplesResult<()> {
        self.sign_and_send(wrap_get_blocks_request(request), key)
    }

    fn async_get_blocktree(&mut self, request: GetBlocktreeRequest,
                           key: Option<&SecretKey>) -> SimplesResult<()> {
        self.sign_and_send(wrap_get_blocktree_request(request), key)
    }

    fn async_pub_block(&mut self, request: PubBlockRequest,
                       key: Option<&SecretKey>) -> SimplesResult<()> {
        self.sign_and_send(wrap_pub_block_request(request), key)
    }
}

// use std::old_io::timer::sleep;
// use std::time::duration::Duration;
// use std::thread::Thread;

// #[test]
// fn test_receiver() {
//     let server = Thread::scoped(move || {
//         // let mut socket1 = Socket::new(Protocol::Rep).unwrap();
//         // let mut endpoint1 = socket1.bind("inproc://mata1").unwrap();
//         let mut socket2 = Socket::new(Protocol::Pull).unwrap();
//         let mut endpoint2 = socket2.bind("inproc://mata").unwrap();

//         let mut socket_fds = vec![socket2.new_pollfd(true, false)];
//             // socket1.new_pollfd(true, false), socket2.new_pollfd(true, false)];
//         let mut poll_request = PollRequest::new(&mut socket_fds);

//         loop {
//             let poll_result = Socket::poll(&mut poll_request, &Duration::milliseconds(50));
//             if poll_result.is_err() {
//                 if poll_result.err().unwrap().kind == NanoErrorKind::Timeout {
//                     println!("Timeout, re-listening");
//                     continue;
//                 } else {
//                     println!("[app  Failed to read in request '{}'. Exiting.",
//                              poll_result.err().unwrap());
//                     break;
//                 }
//             }
//             println!("Something is ready!");
//             if poll_request.get_fds()[0].can_read() {
//                 println!("server: received {}",
//                          String::from_utf8(socket2.read_to_end().unwrap()).unwrap());
//                 // socket2.write_all(b"pong").unwrap();
//             }
//             // if poll_request.get_fds()[1].can_read() {
//             //     println!("mata2: received {}",
//             //              String::from_utf8(socket2.read_to_end().unwrap()).unwrap());
//             // }
//         }
//     });

//     let client1 = Thread::scoped(move || {
//         let mut socket = Socket::new(Protocol::Req).unwrap();
//         let mut endpoint = socket.connect("inproc://mata").unwrap();

//         for _ in range(0, 10) {
//             sleep(Duration::seconds(2));
//             println!("client1: sending request to mata1");
//             socket.write_all(b"client1");
//             println!("client1: received {}",
//                      String::from_utf8(socket.read_to_end().unwrap()).unwrap());
//         }
//     });

//     let client2 = Thread::scoped(move || {
//         let mut socket = Socket::new(Protocol::Push).unwrap();
//         let mut endpoint = socket.connect("inproc://mata").unwrap();

//         for _ in range(0, 10) {
//             sleep(Duration::seconds(1));
//             println!("client2: sending request to mata2");
//             socket.write_all(b"client2");
//         }
//     });
//     sleep(Duration::seconds(40))
// }
