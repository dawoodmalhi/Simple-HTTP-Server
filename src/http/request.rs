// use crate::http::request;

use super::method::{Method, MethodError};
use std::{convert::TryFrom, error::Error, fmt::{Display, Debug, Formatter, Result as FmtResult}, str};
use super::QueryString;
use std::str::Utf8Error;
pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = parseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(parseError::InvalidRequest),
        // }

        //Refactored code
        let (method, request) = get_next_word(request).ok_or(parseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(parseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(parseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(parseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        // let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        //Refactored code
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, letter) in request.chars().enumerate() {
        if letter == ' ' || letter == '\r' {
            return Some((&request[..index], &request[index+1..]));
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

impl From<MethodError> for parseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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