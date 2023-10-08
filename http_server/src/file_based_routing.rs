use std::{path::Path, collections::HashMap, fs};

use crate::{request::HttpRequest, query::QueryPath, response::{HttpBody, HttpResponse, HttpVersion}};
use crate::response::*;

pub struct Router<P: AsRef<Path>> {
    path: P
}

impl <P: AsRef<Path>> Router<P> {
    pub fn new(path: P) -> Self {
        Router { path }
    }

    pub fn handle_request(&mut self, request: HttpRequest<QueryPath<String>>) -> HttpResponse<Box<dyn HttpBody>> {
        println!("{:?}", &request.path.path);
        let mut path = self.path.as_ref().join(request.path.path.strip_prefix("/").unwrap());
        if path.extension().is_none() {
            path = path.join("index.html");
        }
        println!("{:?}", &path);

        if path.exists() {
            let file_contents = fs::read_to_string(path).unwrap();
            HttpResponse::new(HttpVersion::Http1_1, Status::Ok, HashMap::new(), Box::new(file_contents))
        } else {
            let body = "File not found!".to_string();
            HttpResponse::new(HttpVersion::Http1_1, Status::NotFound, HashMap::new(), Box::new(body))
        }
    }
}

