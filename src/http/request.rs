use super::method::Method;
use std::{convert::TryFrom, error::Error, fmt::{Display, Debug, Formatter, Result as FmtResult}};
pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = parseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum parseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl parseError {
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl Display for parseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for parseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for parseError {
    
}