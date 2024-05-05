use crate::server::Handler;
use crate::http::{Request, Response, StatusCode};

pub struct PingServer;

impl Handler for PingServer {
    fn handle(&mut self, req: &Request) -> Response {
        let mut resp = Response::new(StatusCode::Ok, Some("{\"message\" : \"pong\"}".to_string()));
        resp.set_header("content-type".to_string(), "application/json".to_string());
        resp
    }
}