use std::path::Path;

use crate::{response::HttpResponse, request::{HttpRequest, Method}};

pub trait RoutePath {
    fn to_path_object(self) -> RoutePathObject;
}

pub struct Router {
}

pub struct RouterBuilder {
}

pub struct RoutePathObject {
}

pub struct RouterPathBuilder<'a, P: RoutePath> {
    builder: &'a mut RouterBuilder,
    path: P,
    method: Option<Method>,
}

#[derive(Default)]
pub struct DefaultRoute {
}

impl Router {
    pub fn new() -> Self {
        Router {  }
    }
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {}
    }

    pub fn route<'a, P: RoutePath>(&'a mut self, path: P) -> RouterPathBuilder<'a, P> {
        RouterPathBuilder { builder: self, path, method: None, }
    }

    pub fn default<'a>(&'a mut self) -> RouterPathBuilder<'a, DefaultRoute> {
        RouterPathBuilder { builder: self, path: DefaultRoute::default(), method: None, }
    }

    pub fn build(self) -> Router {
        Router::new()
    }
}

impl <'a, P: RoutePath> RouterPathBuilder<'a, P> {
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn static_res<R: Into<HttpResponse>>(self, response: R) {
        todo!();
    }

    pub fn defer<R: Into<Router>>(self, router: R) {
        todo!()
    }

    pub fn file<PathType: AsRef<Path>>(self, path: PathType) {
        todo!()
    }

    pub fn handler<H: Fn(HttpRequest) -> HttpResponse>(self, handler: H) {
        todo!()
    }
}

impl RoutePath for &str {
    fn to_path_object(self) -> RoutePathObject {
        todo!();
        RoutePathObject{}
    }
}

impl RoutePath for DefaultRoute {
    fn to_path_object(self) -> RoutePathObject {
        todo!();
        RoutePathObject {  }
    }
}

impl Into<Router> for RouterBuilder {
    fn into(self) -> Router {
        self.build()
    }
}

