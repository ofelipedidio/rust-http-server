pub mod request;
pub mod server;
pub mod router;
pub mod response;

use crate::request::HttpRequest;
use crate::request::Method;
use crate::server::ServerBuilder;
use crate::router::RouterBuilder;
use crate::router::Router;
use crate::response::HttpResponseBuilder;
use crate::response::HttpResponse;
use crate::response::Status;

fn make_api() -> Router {
    let mut router = RouterBuilder::new();

    router.route("/info")
        .static_res(HttpResponse::builder().status(Status::Ok).body("Ok"));

    router.build()
}

fn default_handler(request: HttpRequest) -> HttpResponse {
    HttpResponseBuilder::new()
        .status(Status::NotFound)
        .body("Could not find this file")
        .build()
}

fn main() {
    let api = make_api();

    let mut router = RouterBuilder::new();

    router.route("/api/*").defer(api);
    router.route("/*").method(Method::GET).file("./server");
    router.default().handler(default_handler);

    let server = ServerBuilder::new(router);

}
