use std::{fmt::Debug, path::Path, rc::Rc};

use super::{
    http_service::{ServeFileService, StaticResponseService},
    request::{HttpRequest, Method},
    response::HttpResponse,
    route::{DefaultRoute, RoutePathObject, RoutePathService},
    router::Router,
};

pub struct RouterBuilder {
    routes: Vec<RoutePathService>,
}

pub struct RouterPathBuilder<'a> {
    builder: &'a mut RouterBuilder,
    path: RoutePathObject,
    method: Option<Method>,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder { routes: Vec::new() }
    }

    pub fn route<'a, P: TryInto<RoutePathObject>>(&'a mut self, path: P) -> RouterPathBuilder<'a>
    where
        P::Error: Debug,
    {
        let path = path.try_into().unwrap();
        RouterPathBuilder::new(self, path)
    }

    pub fn default<'a>(&'a mut self) -> RouterPathBuilder<'a> {
        RouterPathBuilder::new(self, DefaultRoute::default().try_into().unwrap())
    }

    pub fn build(self) -> Router {
        Router::new()
    }

    fn insert_handler(&mut self, service: RoutePathService) {
        self.routes.push(service);
    }
}

impl<'a> RouterPathBuilder<'a> {
    pub fn new(builder: &'a mut RouterBuilder, path: RoutePathObject) -> Self {
        RouterPathBuilder {
            builder,
            path,
            method: None,
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn static_res<R: Into<HttpResponse>>(self, response: R) {
        let service = StaticResponseService::new(Rc::new(response.into()));
        let service = RoutePathService::new(self.path, Box::new(service));
        // self.builder.insert_handler(service);
        self.builder.routes.push(service);
    }

    pub fn defer<'b, 'c, R: Into<Router>>(self, router: R) {
        RoutePathService::new_with_deferred(self.path, router.into());
    }

    pub fn file<PathType: AsRef<Path>>(self, path: PathType) {
        let service = ServeFileService::new(path.as_ref().to_path_buf());
        let service = RoutePathService::new(self.path, Box::new(service));
        self.builder.insert_handler(service);
    }

    pub fn handler(self, handler: fn(&HttpRequest) -> HttpResponse) {
        let service = RoutePathService::new(self.path, Box::new(handler));
        self.builder.insert_handler(service);
    }
}

impl <'a> Into<Router> for RouterBuilder {
    fn into(self) -> Router {
        self.build()
    }
}
