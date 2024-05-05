use std::collections::HashMap;
use std::io::{Write, Result as IOResult};


#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
}

pub struct Response{
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response{status_code, body, headers: HashMap::new()}
    }

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        let mut headers = String::new();
        for (k, v) in &self.headers {
            headers = format!("{}{}: {}\r\n", headers, k, v);
        }
        write!(stream, "HTTP/1.1 {} {}\r\n{}\r\n{}", self.status_code as u8, self.status_code.get_message(), headers,body)
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

impl StatusCode {
    fn get_message(&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::BadRequest =>"Bad Request",
        }
    }
}