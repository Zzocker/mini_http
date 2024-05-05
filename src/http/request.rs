use std::{collections::HashMap, str::{self, Utf8Error}};

#[derive(Debug)]
pub struct Request<'buf>{
    method: &'buf str,
    path: &'buf str,
    query: HashMap<&'buf str, QueryValue<'buf>>,
    headers: HashMap<&'buf str, &'buf str>,
    body: &'buf [u8],
}

fn get_next_word(buf: &str) -> Option<(&str, &str)> {
    for (i, c) in buf.chars().enumerate() {
        if c == ' ' {
            return Some((&buf[..i], &buf[i+1..]));
        }else if c == '\r' {
            return Some((&buf[..i], &buf[i+2..]));
        }
    }
    None
}

fn get_next_header(buf: &str) -> Option<(&str, &str)> {
    for (i, c) in buf.chars().enumerate() {
        if c == '\r' {
            return Some((&buf[..i], &buf[i+2..]));
        }
    }
    None
}

/*
    GET /hello?a=1&b=2&c&a=2 HTTP/1.1
    Content-Type: application/json
    User-Agent: PostmanRuntime/7.37.3
    Cache-Control: no-cache
    Postman-Token: 522ee296-6f06-435c-a6f2-65caeee5bdef
    Host: localhost:8080
    Accept-Encoding: gzip, deflate, br
    Connection: keep-alive
    Content-Length: 26

    {
        "message" : "ping"
    }
*/

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = RequestParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let buf = str::from_utf8(buf)?;
        let (method, buf) = get_next_word(buf).ok_or(RequestParseError::InvalidRequest)?;
        let (mut path, buf) = get_next_word(buf).ok_or(RequestParseError::InvalidRequest)?;
        let (protocol, mut buf) = get_next_word(buf).ok_or(RequestParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(RequestParseError::InvalidProtocol);
        }
        let mut query = HashMap::new();
        if let Some(i) = path.find('?') {
            for qs in path[i+1..].split('&') {
                let mut key = qs;
                let mut value = "";
                if let Some(i) = qs.find('=') {
                    key = &qs[..i];
                    value = &qs[i+1..];
                }
                query.entry(key)
                    .and_modify(|existing: &mut QueryValue| match existing {
                        QueryValue::Single(v) => *existing = QueryValue::Multiple(vec![v, value]),
                        QueryValue::Multiple(vec) => vec.push(value)
                    })
                    .or_insert(QueryValue::Single(value));
            }
            path = &path[..i];
        }
        let mut headers = HashMap::new();
        loop {
            let (h, remain) = get_next_header(buf).ok_or(RequestParseError::InvalidRequest)?;
            buf = remain;
            if h.len() == 0 {
                break;
            }
            match h.find(':'){
                Some(i) => {
                    headers.insert(&h[..i], &h[i+2..]);
                },
                None => return Err(RequestParseError::InvalidRequest)
            };
        }

        Ok(Request {
            method: method,
            path: path,
            query: query,
            headers: headers,
            body: buf.as_bytes()
        })
    }
}

#[derive(Debug)]
pub enum RequestParseError{
    InvalidUTF8Encoding,
    InvalidRequest,
    InvalidProtocol
}

impl From<Utf8Error> for RequestParseError {
    fn from(_: Utf8Error) -> Self {
        RequestParseError::InvalidUTF8Encoding
    }
}

#[derive(Debug)]
pub enum QueryValue<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}