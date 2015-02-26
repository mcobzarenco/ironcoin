use std::error::{Error, FromError};
use std::fmt::{self, Display, Formatter};

use nanomsg;
use protobuf;
use rustc_serialize;

pub type IroncResult<Msg> = Result<Msg, IroncError>;

#[derive(Debug, Eq, PartialEq)]
pub struct IroncError {
    description: String
}

impl IroncError {
    pub fn new(description: &str) -> IroncError {
        IroncError { description: String::from_str(description) }
    }
}

impl Display for IroncError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&self.description)
    }
}

impl Error for IroncError {
    fn description(&self) -> &str { &self.description }

    fn cause(&self) -> Option<&Error> { None }
}

// TODO: Use something like below (albeit with a IroncErrorTrait)
//       once Rust gets negative trait bounds
//
// impl<Error: ::std::error::Error + !IroncError>
//     FromError<Error> for IroncError {
//
//     fn from_error(err: Error) -> IroncError {
//         IroncError { description: String::from_str(err.description()) }
//     }
// }

trait ConvertToIroncError: Error {}
impl ConvertToIroncError for protobuf::error::ProtobufError {}
impl ConvertToIroncError for ::std::io::Error {}
impl ConvertToIroncError for rustc_serialize::json::EncoderError {}
impl ConvertToIroncError for rustc_serialize::base64::FromBase64Error {}

impl<Err: ConvertToIroncError> FromError<Err> for IroncError {
    fn from_error(err: Err) -> IroncError {
        IroncError { description: String::from_str(err.description()) }
    }
}

impl FromError<nanomsg::NanoError> for IroncError {
    fn from_error(err: nanomsg::NanoError) -> IroncError {
        IroncError { description: String::from_str(err.description) }
    }
}

impl<'a> FromError<&'a str> for IroncError {
    fn from_error(err: &'a str) -> IroncError {
        IroncError { description: String::from_str(err) }
    }
}
