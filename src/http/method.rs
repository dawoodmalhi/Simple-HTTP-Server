use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    PATCH,
    POST,
    HEAD,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "POST" => Ok(Self::POST),
            "HEAD" => Ok(Self::HEAD),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;