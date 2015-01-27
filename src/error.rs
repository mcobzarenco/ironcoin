use std::error::{self, FromError};

pub type SimplesResult<Msg> = Result<Msg, SimplesError>;

#[derive(Debug, Eq, PartialEq)]
pub struct SimplesError {
    pub description: String
}

impl<Error: ::std::error::Error> FromError<Error> for SimplesError {
    fn from_error(err: Error) -> SimplesError {
        SimplesError { description: String::from_str(err.description()) }
    }
}
