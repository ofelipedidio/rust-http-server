pub mod component;

use crate::component::request::HttpRequest;
use crate::component::request::Method;
use crate::component::server::ServerBuilder;
use crate::component::router_builder::RouterBuilder;
use crate::component::router::Router;
use crate::component::response::HttpResponseBuilder;
use crate::component::response::HttpResponse;
use crate::component::response::Status;

fn make_api() -> Router {
    let mut router = RouterBuilder::new();

    router.route("/info")
        .static_res(HttpResponseBuilder::new().status(Status::Ok).body("Ok"));

    router.build()
}

fn default_handler(_request: &HttpRequest) -> HttpResponse {
    HttpResponseBuilder::new()
        .status(Status::NotFound)
        .body("Could not find this file")
        .build()
}

fn main() {
    let api = make_api();

    let mut router = RouterBuilder::new();

    router.route("api").defer(api);
    router.route("*").method(Method::GET).file("./server");
    router.route("test")
        .defer({
            let mut router = RouterBuilder::new();

            router.route("in")
                .method(Method::GET)
                .static_res(HttpResponseBuilder::new().status(Status::Ok).body("in"));
            router.route("out")
                .static_res(HttpResponseBuilder::new().status(Status::Ok).body("out"));

            router
        });
    router.default().handler(default_handler);

    let server = ServerBuilder::new()
        .router(router)
        .build();
    
    server.serve("0.0.0.0:8080");
}

