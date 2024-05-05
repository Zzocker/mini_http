use crate::server::Handler;
use crate::http::{Request, Response, StatusCode, Method};
use serde::Deserialize;

pub struct PingServer{
    x: i32
}

impl PingServer {
    
    pub fn new() -> Self {
        PingServer{x: 0}
    }
}


#[derive(Deserialize, Debug)]
struct UpdateValueDto {
    value: i32
}

impl Handler for PingServer {
    fn handle(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => Response::new(StatusCode::Ok, Some(self.x.to_string())),
            Method::PUT => {
                // set x 
                match serde_json::from_slice::<UpdateValueDto>(req.body()) {
                    Ok(dto) => {
                        self.x=dto.value;
                        Response::new(StatusCode::Ok, Some(self.x.to_string()))
                    }
                    Err(err) => Response::new(StatusCode::BadRequest, None)
                }
            },
            Method::POST => {
                match req.path() {
                    "/inc" => {
                        // inc x
                        match serde_json::from_slice::<UpdateValueDto>(req.body()) {
                            Ok(dto) => {
                                self.x+=dto.value;
                                Response::new(StatusCode::Ok, Some(self.x.to_string()))
                            }
                            Err(err) => Response::new(StatusCode::BadRequest, None)
                        }
                    }
                    _ => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}