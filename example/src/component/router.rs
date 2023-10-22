use std::{rc::Rc, collections::HashMap, error::Error};

use crate::{
    component::request::HttpRequest,
    component::response::HttpResponse,
};

use super::{http_service::HttpService, response::{HttpResponseBuilder, Status}};

pub enum Router {
    Router{
        paths: HashMap<String, Vec<Router>>,
        default_services: Vec<Box<dyn HttpService>>,
    },
    Service(Option<Method>, Box<dyn HttpService>),
}

fn compute_segments<'a>(path: &'a [char]) -> (&'a [char], &'a [char]) {
    let mut i = 0;

    // Ignore leading slashes
    while i < path.len() {
        match path[i] {
            '/' => i += 1,
            _ => break,
        }
    }

    // Take the first path segment
    let seg_init = i;
    while i < path.len() {
        match path[i] {
            '/' => break,
            _ => i += 1,
        }
    }

    let prefix = &path[seg_init..i];
    let suffix = &path[i..];

    (prefix, suffix)
}

impl Router {
    pub fn new(paths: HashMap<String, Vec<Router>>, default_services: Vec<Box<dyn HttpService>>) -> Self {
        Router::Router{ paths, default_services }
    }

    pub fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        let path: Vec<_> = request.path().chars().collect();
        self.handle_internal(request, &path)
    }

    fn handle_internal(&mut self, request: &HttpRequest, path: &[char]) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        // TODO: Find a way to propagate the current path to the request handler
        match self {
            Self::Router{paths, default_services} => {
                let (prefix, suffix) = compute_segments(&path);

                let prefix: String = prefix.into_iter().collect();
                match paths.get_mut(&prefix) {
                    Some(next) => {
                        for handler in next {
                            match handler.handle_internal(request, suffix) {
                                Ok(response) => return Ok(response),
                                Err(_) => continue,
                            }
                        }

                        for handler in default_services.iter_mut() {
                            match handler.handle(request) {
                                Ok(response) => return Ok(response),
                                Err(_) => continue,
                            }
                        }

                        Ok(Rc::new(HttpResponseBuilder::new().status(Status::NotFound).body("404 Not Found").build()))
                    }
                    None => {
                        for handler in default_services.iter_mut() {
                            match handler.handle(request) {
                                Ok(response) => return Ok(response),
                                Err(_) => continue,
                            }
                        }

                        Ok(Rc::new(HttpResponseBuilder::new().status(Status::NotFound).body("404 Not Found").build()))
                    }
                }
            }
            Self::Service(method, service) => {
                match method {
                    Some(method) => {
                        if request.method() == method {
                            return Ok(service.handle(request)?)
                        } else {
                            return Err(todo!("Create next fallback"));
                        }
                    }
                    None => {
                        return Ok(service.handle(request)?)
                    }
                }
            }
        }
    }
}

