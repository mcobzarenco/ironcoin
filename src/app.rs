use std::error::FromError;
use std::time::Duration;

use nanomsg::{Endpoint, PollRequest, Protocol, Socket, NanoErrorKind};
use protobuf::{self, Message};
use rustc_serialize::json;

use block::HashedBlockExt;
use crypto::HashDigest;
use error::{SimplesResult};
use service::{HeadBlockPubService, RpcService, StakerService};
use simples_pb::{
    self, RpcRequest, RpcRequest_Method, RpcResponse, RpcResponse_Status,
    PublishTransactionRequest, PublishTransactionResponse,
    PublishBlockRequest, PublishBlockResponse, HashedBlock,
    Wallet};
use staking::{BlockTemplate, Staker, STAKING_INTERVAL};
use wallet::WalletExt;

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

    fn dispatch(&mut self, request: RpcRequest) -> SimplesResult<RpcResponse> {
        let request_bytes = try!(request.write_to_bytes());
        try!(self.socket.write_all(&request_bytes[]));

        let response_bytes = try!(self.socket.read_to_end());
        Ok(try!(protobuf::parse_from_bytes(&response_bytes[])))
    }
}

impl Drop for Client {
    fn drop(&mut self) { self.endpoint.shutdown().unwrap(); }
}

impl RpcService for Client {
    fn pub_block(&mut self, request: PublishBlockRequest) ->
        SimplesResult<PublishBlockResponse>
    {
        let mut wrapped_request = RpcRequest::new();
        wrapped_request.set_method(RpcRequest_Method::PUBLISH_BLOCK);
        wrapped_request.set_pub_block(request);
        self.dispatch(wrapped_request)
            .map(|mut response| response.take_pub_block())
    }

    fn pub_transaction(&mut self, request: PublishTransactionRequest) ->
        SimplesResult<PublishTransactionResponse>
    {
        let mut wrapped_request = RpcRequest::new();
        wrapped_request.set_method(RpcRequest_Method::PUBLISH_TRANSACTION);
        wrapped_request.set_pub_transaction(request);
        self.dispatch(wrapped_request)
            .map(|mut response| response.take_pub_transaction())
    }
}

pub struct Application<Service: RpcService + StakerService + HeadBlockPubService> {
    pub endpoint: String,
    peers: Vec<Client>,
    service: Service,
    staker: Staker,
    sub_head_socket: Socket,
    sub_head_endpoint: Endpoint
}

impl<Service: RpcService + StakerService + HeadBlockPubService>
    Application<Service> {
    pub fn new(endpoint: &str, service: Service, peers: Vec<String>,
               staking_keys: Wallet)
               -> SimplesResult<Application<Service>>
    {
        let mut sub_head_socket = try!(Socket::new(Protocol::Sub));
        try!(sub_head_socket.subscribe(""));
        let sub_head_endpoint = try!(
            sub_head_socket.connect(service.get_pub_endpoint()));
        let staker = Staker::new(
            try!(HashDigest::from_bytes(service.current_head_block().get_hash())),
            service.current_head_block().get_block().get_timestamp());

        let mut peer_clients = Vec::<Client>::new();
        for endpoint in peers.iter() {
            peer_clients.push(try!(Client::new(endpoint)));
        }

        Ok(Application {
            endpoint: String::from_str(endpoint),
            peers: peer_clients,
            service: service,
            staker: staker,
            sub_head_socket: sub_head_socket,
            sub_head_endpoint: sub_head_endpoint
        })
    }

    fn dispatch(&mut self, mut request: RpcRequest) -> RpcResponse {
        let mut response = RpcResponse::new();
        match request.get_method() {
            RpcRequest_Method::INVALID => {
                println!("Peer tried to call invalid method.");
                response.set_status(RpcResponse_Status::INVALID_METHOD);
                response
            },
            RpcRequest_Method::PUBLISH_TRANSACTION => {
                response.set_status(RpcResponse_Status::OK);
                response.set_pub_transaction(
                    self.service.pub_transaction(
                        request.take_pub_transaction()).unwrap());
                response
            },
            RpcRequest_Method::PUBLISH_BLOCK => {
                response.set_status(RpcResponse_Status::OK);
                response.set_pub_block(
                    self.service.pub_block(request.take_pub_block()).unwrap());
                response
            }
        }
    }

    fn handle_raw_request(&mut self, raw_request: &[u8]) -> RpcResponse {
        match protobuf::parse_from_bytes(&raw_request[]) {
            Ok(request) => self.dispatch(request),
            Err(err) => {
                println!("[app loop] Failed to parse protobuf '{:?}'.", err);
                let mut response = simples_pb::RpcResponse::new();
                response.set_status(
                    simples_pb::RpcResponse_Status::INVALID_MESSAGE);
                response
            }
        }
    }

    pub fn run(&mut self) -> SimplesResult<()> {
        let mut rpc_socket = try!(Socket::new(Protocol::Rep));
        let mut rpc_endpoint = try!(rpc_socket.bind(&self.endpoint[]));
        let mut poll_fds = [rpc_socket.new_pollfd(true, false),
                            self.sub_head_socket.new_pollfd(true, false)];
        let mut poll_request = PollRequest::new(&mut poll_fds[]);

        println!("[app loop] RPC service is running at {}", self.endpoint);
        loop {
            let poll_result = Socket::poll(
                &mut poll_request, &Duration::seconds(STAKING_INTERVAL));
            if poll_result.is_err() {
                let err = poll_result.err().unwrap();
                if err.kind == NanoErrorKind::Timeout {
                    try!(self.handle_timeout());
                    continue;
                } else {
                    println!("[app loop] Failed to read in request '{}'. Exiting.",
                             err);
                    return Err(FromError::from_error(err));
                }
            }
            if poll_request.get_fds()[0].can_read() { // Rpc Request
                match rpc_socket.read_to_end() {
                    Ok(request) => {
                        let response = self.handle_raw_request(&request[]);
                        let response_bytes = response.write_to_bytes().unwrap();
                        let write_result = rpc_socket.write_all(&response_bytes[]);
                        if write_result.is_err() {
                            let err = write_result.err().unwrap();
                            println!("[app loop] Failed to send response '{}'. Exiting.",
                                     err);
                        }
                    },
                    Err(err) => {
                        println!(
                            "[app loop] Failed to read in request '{}'. Exiting.",
                            err);
                        return Err(FromError::from_error(err));
                    }
                }
            }
            if poll_request.get_fds()[1].can_read() { // New head block
                match self.sub_head_socket.read_to_end() {
                    Ok(msg) => {
                        self.handle_new_head_block(
                            try!(protobuf::parse_from_bytes(&msg[])));
                    },
                    Err(err) => {
                        println!(
                            "[app loop] Failed to read in published new block '{}'.",
                            err);
                        return Err(FromError::from_error(err));
                    }
                };
            }
        } // loop
        try!(rpc_endpoint.shutdown());
    }

    fn handle_timeout(&mut self) -> SimplesResult<()> {
        match self.staker.stake_interval(STAKING_INTERVAL) {
            Some(template) => self.service.on_successful_stake(template),
            None => Ok(())
        }
    }

    fn handle_new_head_block(&mut self, new_head: HashedBlock)
                             -> SimplesResult<()>
    {
        println!("Sub-ed new block!");
        self.staker.set_head_block(
            try!(HashDigest::from_bytes(new_head.get_hash())),
            new_head.get_block().get_timestamp());

        for peer_index in range(0, self.peers.len()) {
            let mut request = PublishBlockRequest::new();
            request.set_block(new_head.clone());
            let response = self.peers[peer_index].pub_block(request);
            if response.is_err() {
                println!("Could not pub_block to peer \"{}\":P{}",
                         self.peers[peer_index].endpoint_str,
                         response.as_ref().err().unwrap());
            }
            println!("pub_block: got status {:?}", response.unwrap().get_status());
        }
        Ok(())
    }
}

#[unsafe_destructor]
impl<Service: RpcService + StakerService + HeadBlockPubService> Drop
    for Application<Service> {
    fn drop(&mut self) { self.sub_head_endpoint.shutdown().unwrap(); }
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
//         let mut poll_request = PollRequest::new(&mut socket_fds[]);

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
