use std::net::TcpListener;

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
                Ok((_, _)) => {
                    println!("receive client connection")
                },
                Err(err) => eprint!("failed to accept client connection: {}", err)
            };
        }
    }
}