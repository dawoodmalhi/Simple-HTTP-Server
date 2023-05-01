use crate::http::{Request};
use std::{net::TcpListener, io::Read, convert::{TryFrom}};
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => println!("Failed to parse a request: {}", e)
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