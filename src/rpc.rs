use nanomsg::{Endpoint, Protocol, Socket};
use protobuf::{self, Message};
use std::error::FromError;

use error::{SimplesResult};
use service;
use simples_pb::{
    self, RpcRequest, RpcRequest_Method, RpcResponse, RpcResponse_Status,
    PublishTransactionRequest, PublishTransactionResponse};

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

impl service::Service for Client {
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

pub struct Application<Service: service::Service> {
    pub endpoint_str: String,
    service: Service
}

impl<Service: service::Service> Application<Service> {
    pub fn new(endpoint_str: &str, service: Service) ->
        SimplesResult<Application<Service>>
    {
        Ok(Application {
            endpoint_str: String::from_str(endpoint_str),
            service: service
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
                        request.take_pub_transaction()).ok().unwrap());
                response
            }
        }
    }

    pub fn run(&mut self) -> SimplesResult<()> {
        let mut socket = try!(Socket::new(Protocol::Rep));
        println!("endpoint [{}]: {}", self.endpoint_str.len(), self.endpoint_str);
        let mut endpoint = try!(socket.bind(&self.endpoint_str[]));

        println!("[app loop] RPC service is running at {}", self.endpoint_str);
        loop { match socket.read_to_end() {
            Ok(request_bytes) => {
                println!("Received request!");
                let response =
                    match protobuf::parse_from_bytes(&request_bytes[]) {
                        Ok(request) => self.dispatch(request),
                        Err(err) => {
                            println!("[app loop] Failed to parse protobuf '{:?}'.",
                                     err);
                            let mut response = simples_pb::RpcResponse::new();
                            response.set_status(
                                simples_pb::RpcResponse_Status::INVALID_MESSAGE);
                            response
                        }
                    };
                let response_bytes = response.write_to_bytes().unwrap();
                match socket.write_all(&response_bytes[]) {
                    Ok(..) => (),
                    Err(err) => {
                        println!("[app loop] Failed to send response '{}'.",
                                 err);
                    }
                };
            },
            Err(err) => {
                println!("[app loop] Failed to read in request '{}'. Exiting.",
                         err);
                return Err(FromError::from_error(err))
            }
        } // match
        } // loop
        try!(endpoint.shutdown());
    }
}
