use std::{rc::Rc, path::PathBuf};

use crate::component::response::{HttpResponseBuilder, Status};

use super::{request::HttpRequest, response::HttpResponse, router::Router, route::{RoutePathObject, PathSegment}};

pub trait HttpService {
    fn handle(&mut self, request: &HttpRequest) -> Rc<HttpResponse>;
}

pub struct ServeFileService(PathBuf);

impl  HttpService for fn(&HttpRequest) -> HttpResponse {
    fn handle(&mut self, request: &HttpRequest) -> Rc<HttpResponse> {
        Rc::new(self(request))
    }
}

impl  HttpService for fn(&HttpRequest) -> Rc<HttpResponse> {
    fn handle(&mut self, request: &HttpRequest) -> Rc<HttpResponse> {
        self(request)
    }
}

impl ServeFileService {
    pub fn new(path: PathBuf) -> Self {
        ServeFileService(path)
    }
}

impl  HttpService for ServeFileService {
    fn handle(&mut self, request: &HttpRequest) -> Rc<HttpResponse> {
        let path_segments = request.path().split("/").filter(|x| x.len() > 0);

        let mut path = self.0.to_path_buf();
        for segment in path_segments {
            path = path.join(segment);
        }

        let os_path = path.as_path();

        if !os_path.starts_with(self.0.as_path()) {
            return Rc::new(HttpResponseBuilder::new()
                .status(Status::Forbidden)
                .body(
                    "403 Forbidden\n\nReason: Attempted to read a file outside the server sandbox!",
                )
                .build());
        }

        use std::fs::read_to_string;
        let file = read_to_string(os_path);

        let response = match file {
            Ok(file) => HttpResponseBuilder::new()
                .status(Status::Ok)
                .body(file)
                .build(),
            Err(err) => HttpResponseBuilder::new()
                .status(Status::NotFound)
                .body(format!("{err:#?}"))
                .build(),
        };

        return Rc::new(response);
    }
}

pub struct DeferredRouteService {
    path: RoutePathObject,
    router: Router,
}

impl DeferredRouteService {
    pub fn new(path: RoutePathObject, router: Router) -> Self {
        DeferredRouteService { path, router }
    }
}

fn remove_prefix(request_path: &str, route_path: &RoutePathObject) -> String {
    let mut i = 0usize;

    let mut path = request_path.chars().into_iter();

    for segment in route_path.path().into_iter() {
        match segment {
            PathSegment::Literal(s) => {
                for c in s.chars().into_iter() {
                    match path.next() {
                        Some(cc) if cc == c => i += 1,
                        Some(cc) => todo!("Wrong char on remove_prefix (expected {c:?}, found {cc:?}, at index {i}) (I)"),
                        None => todo!("request_path ended too early on remove_prefix (expected {c:?}, found EOF, at index {i}) (I)"),
                    }
                }

                match path.next() {
                    Some('/') => i += 1,
                    Some(cc) => todo!("Wrong char on remove_prefix (expected '/', found {cc:?}, at index {i}) (II)"),
                    None => todo!("request_path ended too early on remove_prefix (expected '/', found EOF, at index {i}) (II)"),
                }
            }
            PathSegment::Wildcard => {
                return path.collect();
            }
        }
    }

    return String::new();
}

impl  HttpService for DeferredRouteService {
    fn handle(&mut self, request: &HttpRequest) -> Rc<HttpResponse> {
        let new_path = remove_prefix(request.path().as_str(), &self.path);
        let request = HttpRequest::new(request.method(), &new_path, request.query(), request.headers(), request.body());
        self.router.handle(&request)
    }
}

pub struct StaticResponseService{
    response: Rc<HttpResponse>, 
}

impl StaticResponseService {
    pub fn new(response: Rc<HttpResponse>) -> Self {
        StaticResponseService { response }
    }
}

impl  HttpService for StaticResponseService {
    fn handle(&mut self, _request: &HttpRequest) -> Rc<HttpResponse> {
        self.response.clone()
    }
}


