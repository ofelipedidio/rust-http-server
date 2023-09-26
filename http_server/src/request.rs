use std::collections::HashMap;

use crate::path::Path;

#[derive(Debug)]
pub enum Method {
    Get,
    Custom(String),
}

impl Method {
    pub fn from_str<S: AsRef<str>>(string: S) -> Method {
        match string.as_ref() {
            "GET" => Method::Get,
            string => Method::Custom(string.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest<P: Path> {
    pub http_version: String,
    pub path: P,
    pub method: Method,
    pub headers: HashMap<String, String>,
}

impl <P: Path> HttpRequest<P> {
    pub fn parse_request(status_line: String, header_lines: Vec<String>) -> Self {
        // Parse status line
        let mut status_line = status_line.splitn(2, " ");
        let method = status_line.next().unwrap();
        let method = Method::from_str(method);
        let status_line = status_line.next().unwrap();
        let mut status_line = status_line.rsplitn(2, " ");
        let http_version = status_line.next().unwrap().to_string();
        let path = status_line.next().unwrap().to_string();
        let path = P::parse(path).unwrap();

        // Parse headers
        let mut headers = HashMap::new();
        for line in header_lines.into_iter() {
            let mut line = line.splitn(2, ": ");
            let key = line.next().unwrap();
            let value = line.next().unwrap();
            headers.insert(key.to_string(), value.to_string());
        }

        HttpRequest { 
            http_version, 
            path, 
            method, 
            headers
        }
    }
}

