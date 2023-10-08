use std::{error::Error, net::TcpStream, collections::HashMap, io::Write};

pub trait HttpBody {
    fn write_headers(&self, headers: &mut HashMap<String, String>);
    fn write_body(self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>>;
}


pub struct HttpResponse {
}

pub struct HttpResponseBuilder {
    status: Option<Status>,
    body: Option<Box<dyn HttpBody>>,
}

pub enum Status {
    Ok,
    NotFound,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {  }
    }

    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::new()
    }
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        HttpResponseBuilder { status: None, body: None }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn body<B: HttpBody + 'static>(mut self, body: B) -> Self {
        self.body = Some(Box::new(body));
        self
    }

    pub fn build(self) -> HttpResponse {
        HttpResponse {  }
    }
}

impl Into<HttpResponse> for HttpResponseBuilder {
    fn into(self) -> HttpResponse {
        self.build()
    }
}

impl HttpBody for &str {
    fn write_headers(&self, headers: &mut HashMap<String, String>) {
        headers.insert("Content-Length".into(), self.as_bytes().len().to_string());
    }

    fn write_body(self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        Ok(stream.write_all(self.as_bytes())?)
    }
}

