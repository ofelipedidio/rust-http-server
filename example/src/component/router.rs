use std::rc::Rc;

use crate::{
    component::request::HttpRequest,
    component::response::HttpResponse,
};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Router {}
    }

    pub fn handle(&self, request: &HttpRequest) -> Rc<HttpResponse> {
        todo!()
    }
}

