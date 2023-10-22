use std::{fmt::Debug, path::Path, rc::Rc, collections::HashMap};

use super::{
    http_service::{ServeFileService, StaticResponseService},
    request::{HttpRequest, Method},
    response::{HttpResponse, HttpResponseBuilder, Status},
    router::Router,
};

pub struct RouterBuilder {
    routes: Vec<RoutePathService>,
}

#[derive(Default)]
pub struct NoMethod;

pub struct RouterPathBuilder<'a, M> {
    builder: &'a mut RouterBuilder,
    path: PathSegment,
    method: M,
}

pub struct RoutePathService {
    path: PathSegment,
    service: Router,
}

impl RoutePathService {
    pub fn new(path: PathSegment, service: Router) -> Self {
        RoutePathService { path, service }
    }

    pub fn path(&self) -> &PathSegment {
        &self.path
    }

    pub fn service(&self) -> &Router {
        &self.service
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PathSegment {
    Literal(String),
    Wildcard,
}

impl PathSegment {
    fn from<S: Into<String>>(seg: S) -> Self {
        let seg = seg.into();
        match seg.as_str() {
            "*" => PathSegment::Wildcard,
            _ => PathSegment::Literal(seg),
        }
    }
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder { routes: Vec::new() }
    }

    pub fn route<'a, P: TryInto<PathSegment>>(&'a mut self, path: P) -> RouterPathBuilder<'a, NoMethod>
    where
        P::Error: Debug,
    {
        let path = path.try_into().unwrap();
        RouterPathBuilder::new(self, path)
    }

    pub fn default<'a>(&'a mut self) -> RouterPathBuilder<'a, NoMethod> {
        RouterPathBuilder::new(self, PathSegment::Wildcard)
    }

    pub fn build(self) -> Router {
        let mut paths = HashMap::new();
        let mut default_service = None;

        for route in self.routes.into_iter() {
            let RoutePathService {path, service} = route;

            match path {
                PathSegment::Literal(path) => {
                    paths.insert(path, service);
                }
                PathSegment::Wildcard => {
                    default_service = Some(service);
                }
            }
        }
        
        let default_service = if let Some(service) = default_service {
            service
        } else {
            // Construct default service
            let response = HttpResponseBuilder::new()
                .status(Status::NotFound)
                .body("Invalid path")
                .build();
            let response = Rc::new(response);
            let service = StaticResponseService::new(response);
            Router::Service(Box::new(service))
        };

        Router::new(paths, default_service)
    }

    fn insert_handler(&mut self, service: RoutePathService) {
        self.routes.push(service);
    }
}

impl<'a> RouterPathBuilder<'a, NoMethod> {
    pub fn new(builder: &'a mut RouterBuilder, path: PathSegment) -> Self {
        RouterPathBuilder {
            builder,
            path,
            method: NoMethod::default(),
        }
    }

    pub fn method(self, method: Method) -> RouterPathBuilder<'a, Method> {
        RouterPathBuilder{
            builder: self.builder,
            path: self.path,
            method,
        }
    }
}

impl<'a, M> RouterPathBuilder<'a, M> {
    pub fn static_res<R: Into<HttpResponse>>(self, response: R) {
        let service = StaticResponseService::new(Rc::new(response.into()));
        let service = Box::new(service);
        let service = Router::Service(service);
        let service = RoutePathService::new(self.path, service);
        self.builder.insert_handler(service);
    }

    pub fn defer<'b, 'c, R: Into<Router>>(self, router: R) {
        let service = RoutePathService::new(self.path, router.into());
        self.builder.insert_handler(service);
    }

    pub fn file<PathType: AsRef<Path>>(self, path: PathType) {
        let service = ServeFileService::new(path.as_ref().to_path_buf());
        let service = Box::new(service);
        let service = Router::Service(service);
        let service = RoutePathService::new(self.path, service);
        self.builder.insert_handler(service);
    }

    pub fn handler(self, handler: fn(&HttpRequest) -> HttpResponse) {
        let service = Box::new(handler);
        let service = Router::Service(service);
        let service = RoutePathService::new(self.path, service);
        self.builder.insert_handler(service);
    }
}

impl <'a> Into<Router> for RouterBuilder {
    fn into(self) -> Router {
        self.build()
    }
}

impl From<&str> for PathSegment {
    fn from(value: &str) -> Self {
        match value {
            "*" => PathSegment::Wildcard,
            value => PathSegment::Literal(value.to_string()),
        }
    }
}

