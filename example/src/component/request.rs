use std::collections::HashMap;

pub enum Method {
    GET,
}

pub struct HttpRequest<'a> {
    method: &'a Method,
    path: &'a String,
    query: &'a HashMap<String, String>,
    headers: &'a HashMap<String, String>,
    body: &'a Vec<u8>,
}

impl <'a> HttpRequest<'a> {
    pub fn new(method: &'a Method, path: &'a String, query: &'a HashMap<String, String>, headers: &'a HashMap<String, String>, body: &'a Vec<u8>) -> Self {
        HttpRequest { 
            method, 
            path,
            query,
            headers,
            body
        }
    }

    pub fn method(&self) -> &Method {
        self.method
    }

    pub fn path(&self) -> &String {
        self.path
    }

    pub fn query(&self) -> &HashMap<String, String> {
        self.query
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        self.headers
    }

    pub fn body(&self) -> &Vec<u8> {
        self.body
    }
}
