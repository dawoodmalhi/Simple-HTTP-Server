use crate::http::{Request, Response, StatusCode, status_code};
use std::{net::TcpListener, io::{Read, Write}, convert::{TryFrom}, fmt::write};
pub struct Server{
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self{
        Self{
            addr
        }
    }
    pub fn run(self){
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
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("<h1>Aroma. Success</h1>".to_string()))
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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