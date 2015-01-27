use error::{SimplesResult};
use simples_pb;

pub trait SimplesService {
    fn pub_transaction(
        &mut self, request: simples_pb::PublishTransactionRequest) ->
        SimplesResult<simples_pb::PublishTransactionResponse>;
}
