use std::{rc::Rc, path::PathBuf, error::Error};

use crate::component::response::{HttpResponseBuilder, Status};

use super::{request::HttpRequest, response::HttpResponse, router::Router, router_builder::PathSegment};

pub trait HttpService {
    fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>>;
}

impl HttpService for Box<dyn HttpService> {
    fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        self.as_mut().handle(request)
    }
}

pub struct ServeFileService(PathBuf);

impl  HttpService for fn(&HttpRequest) -> HttpResponse {
    fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        Ok(Rc::new(self(request)))
    }
}

impl  HttpService for fn(&HttpRequest) -> Rc<HttpResponse> {
    fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        Ok(self(request))
    }
}

impl ServeFileService {
    pub fn new(path: PathBuf) -> Self {
        ServeFileService(path)
    }
}

impl  HttpService for ServeFileService {
    fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        let path_segments = request.path().split("/").filter(|x| x.len() > 0);

        let mut path = self.0.to_path_buf();
        for segment in path_segments {
            path = path.join(segment);
        }

        let os_path = path.as_path();

        if !os_path.starts_with(self.0.as_path()) {
            return Ok(Rc::new(HttpResponseBuilder::new()
                .status(Status::Forbidden)
                .body(
                    "403 Forbidden\n\nReason: Attempted to read a file outside the server sandbox!",
                )
                .build()));
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

        return Ok(Rc::new(response));
    }
}

pub struct DeferredRouteService {
    path: PathSegment,
    router: Router,
}

impl DeferredRouteService {
    pub fn new(path: PathSegment, router: Router) -> Self {
        DeferredRouteService { path, router }
    }
}

impl HttpService for DeferredRouteService {
    fn handle(&mut self, request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        todo!("Handle deferred route service");
        // let new_path = remove_prefix(request.path().as_str(), &self.path);
        // let request = HttpRequest::new(request.method(), &new_path, request.query(), request.headers(), request.body());
        // self.router.handle(&request)
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

impl HttpService for StaticResponseService {
    fn handle(&mut self, _request: &HttpRequest) -> Result<Rc<HttpResponse>, Box<dyn Error>> {
        Ok(self.response.clone())
    }
}


