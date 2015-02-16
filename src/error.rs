use std::error::{Error, FromError};
use std::fmt::{self, Display, Formatter};

use nanomsg;
use protobuf;
use rustc_serialize;

pub type SimplesResult<Msg> = Result<Msg, SimplesError>;

#[derive(Debug, Eq, PartialEq)]
pub struct SimplesError {
    description: String
}

impl SimplesError {
    pub fn new(description: &str) -> SimplesError {
        SimplesError { description: String::from_str(description) }
    }
}

impl Display for SimplesError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&self.description[])
    }
}

impl Error for SimplesError {
    fn description(&self) -> &str { &self.description[] }

    fn cause(&self) -> Option<&Error> { None }
}

// TODO: Use something like below (albeit with a SimplesErrorTrait)
//       once Rust gets negative trait bounds
//
// impl<Error: ::std::error::Error + !SimplesError>
//     FromError<Error> for SimplesError {
//
//     fn from_error(err: Error) -> SimplesError {
//         SimplesError { description: String::from_str(err.description()) }
//     }
// }

trait ConvertToSimplesError: Error {}
impl ConvertToSimplesError for protobuf::error::ProtobufError {}
impl ConvertToSimplesError for ::std::old_io::IoError {}
impl ConvertToSimplesError for rustc_serialize::json::EncoderError {}
impl ConvertToSimplesError for rustc_serialize::base64::FromBase64Error {}

impl<Err: ConvertToSimplesError> FromError<Err> for SimplesError {
    fn from_error(err: Err) -> SimplesError {
        SimplesError { description: String::from_str(err.description()) }
    }
}

impl FromError<nanomsg::NanoError> for SimplesError {
    fn from_error(err: nanomsg::NanoError) -> SimplesError {
        SimplesError { description: String::from_str(err.description) }
    }
}

impl<'a> FromError<&'a str> for SimplesError {
    fn from_error(err: &'a str) -> SimplesError {
        SimplesError { description: String::from_str(err) }
    }
}
