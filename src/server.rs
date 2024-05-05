use std::{io::Read, net::TcpListener};

use crate::http::Request;

pub struct  Server{
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {addr}
    }

    pub fn start(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("listening on {}", self.addr);
        loop {
            // blocking
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("receive client connection");
                    let mut buf = [0; 1024]; // 1 kb buffer to read request
                    match stream.read(&mut buf){
                        Ok(n) => {
                            println!("read client request of size: {} bytes", n);
                            match Request::try_from(&buf[..n]) {
                                Ok(req) => {
                                    dbg!(req);
                                },
                                Err(err) => eprintln!("failed to parse client request: {:?}", err)
                            }
                        },
                        Err(err) => eprintln!("failed to read client request: {}", err)
                    }
                },
                Err(err) => eprintln!("failed to accept client connection: {}", err)
            };
        }
    }
}