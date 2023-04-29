use crate::http::request;

use super::method::Method;
use std::{convert::TryFrom, error::Error, fmt::{Display, Debug, Formatter, Result as FmtResult}, str};
use std::str::Utf8Error;
pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = parseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        // // 1. Original Long code
        // match str::from_utf8(buffer) {
        //     Ok(request) => {}
        //     Err(_) => return Err(parseError::InvalidEncoding),
        // }
        // // 2. Refactored code
        // match str::from_utf8(buffer).or(Err(parseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // // 3. Refactored Shorter code
        // let request = str::from_utf8(buffer).or(Err(parseError::InvalidEncoding))?;

        // 4. Refactor shortest code
        let request = str::from_utf8(buffer)?;


        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, letter) in request.chars().enumerate() {
        let word: String;
        if letter == ' '{
            return Some((&request[..index], &request[index+1..]))
        }
    }
    None
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

impl From<Utf8Error> for parseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
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