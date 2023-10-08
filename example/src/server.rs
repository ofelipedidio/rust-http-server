use crate::router::Router;

pub struct HttpServer {
}

pub struct ServerBuilder {
    router: Router,
}

impl ServerBuilder {
    pub fn new<R: Into<Router>>(router: R) -> Self {
        ServerBuilder {
            router: router.into()
        }
    }

    pub fn build() -> HttpServer {
        todo!()
    }
}


