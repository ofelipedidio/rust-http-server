use std::net::ToSocketAddrs;

use crate::component::router::Router;

pub struct HttpServer {
}

pub struct ServerBuilder {
    router: Option<Router>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder {
            router: None 
        }
    }

    pub fn router<R: Into<Router>>(mut self, router: R) -> Self {
        self.router = Some(router.into());
        self
    }

    pub fn build(self) -> HttpServer {
        todo!("Build http server")
    }
}

impl HttpServer {
    pub fn serve<A: ToSocketAddrs>(self, addr: A) {
        todo!("Serve http server")
    }
}

