use ping_server::PingServer;
use server::Server;

mod server;
mod http;
mod ping_server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.start(PingServer::new());
}
