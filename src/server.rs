use crate::http::{Request, Response, StatusCode, status_code, request, parseError};
use std::{net::TcpListener, io::{Read, Write}, convert::{TryFrom}, fmt::write};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &parseError) -> Response{
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server{
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self{
        Self{
            addr
        }
    }
    pub fn run(self, mut handler: impl Handler){
        println!("Listening on: {}", self.addr);
        let listner = TcpListener::bind(&self.addr).unwrap();
        
        loop {
            match   listner.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                print!("Failed to sent the response: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from conection!: {}", e)
                    }
                }
                Err(e) => println!("Establishing connection failed!: {}", e)
            }
        }
    }
}