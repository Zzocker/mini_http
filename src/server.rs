use std::{io::Read, net::TcpListener};

use crate::http::{Request, RequestParseError, Response, StatusCode};

pub trait Handler{
    fn handle(&mut self, req: &Request) -> Response;
    fn handle_parse_error(&mut self, err: &RequestParseError) -> Response {
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct  Server{
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {addr}
    }

    pub fn start(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("listening on {}", self.addr);
        loop {
            // blocking
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024]; // 1 kb buffer to read request
                    match stream.read(&mut buf){
                        Ok(n) => {
                            let resp = match Request::try_from(&buf[..n]) {
                                Ok(req) => handler.handle(&req),
                                Err(err) => handler.handle_parse_error(&err)
                            };
                            if let Err(err) = resp.send(&mut stream) {
                                eprintln!("failed to send response to client: {}", err);
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