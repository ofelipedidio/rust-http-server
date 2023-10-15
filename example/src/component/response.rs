use std::{error::Error, net::TcpStream, collections::HashMap, io::Write};

pub trait HttpBody {
    fn write_headers(&self, headers: &mut HashMap<String, String>);
    fn write_body(self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>>;
}

pub struct HttpResponse {
    status: Status,
    headers: HashMap<String, String>,
    body: Box<dyn HttpBody>,
}

pub struct NoStatus;
pub struct NoBody;

pub struct HttpResponseBuilder<S, B> {
    status: S,
    headers: HashMap<String, String>,
    body: B,
}

pub enum Status {
    // 2XX
    Ok,

    // 4XX
    Forbidden,
    NotFound,
}

impl Status {
    fn code(&self) -> usize {
        match self {
            Self::Ok => 200,
            Self::Forbidden => 403,
            Self::NotFound => 404,
        }    
    }

    fn message(&self) -> &'static str {
        match self {
            Self::Ok => "Ok",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
        }    
    }
}

impl HttpResponse {
    pub fn new(status: Status, headers: HashMap<String, String>, body: Box<dyn HttpBody>) -> Self {
        HttpResponse {
            status,
            headers,
            body,
        }
    }
}

impl HttpResponseBuilder<NoStatus, NoBody> {
    pub fn new() -> Self {
        HttpResponseBuilder { 
            status: NoStatus,
            headers: HashMap::new(),
            body: NoBody,
        }
    }

}

impl <S, B> HttpResponseBuilder<S, B> {
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

impl <B> HttpResponseBuilder<NoStatus, B> {
    pub fn status(self, status: Status) -> HttpResponseBuilder<Status, B> {
        HttpResponseBuilder {
            status,
            headers: self.headers,
            body: self.body,
        }
    }
}

impl <S> HttpResponseBuilder<S, NoBody> {
    pub fn body<B: HttpBody + 'static>(self, body: B) -> HttpResponseBuilder<S, Box<dyn HttpBody>> {
        HttpResponseBuilder {
            status: self.status,
            headers: self.headers,
            body: Box::new(body),
        }
    }
}

impl HttpResponseBuilder<Status, Box<dyn HttpBody>> {
    pub fn build(self) -> HttpResponse {
        HttpResponse::new(self.status, self.headers, self.body)
    }
}

impl Into<HttpResponse> for HttpResponseBuilder<Status, Box<dyn HttpBody>> {
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

impl HttpBody for String {
    fn write_headers(&self, headers: &mut HashMap<String, String>) {
        headers.insert("Content-Length".into(), self.as_bytes().len().to_string());
    }

    fn write_body(self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        Ok(stream.write_all(self.as_bytes())?)
    }
}

impl HttpBody for &[u8] {
    fn write_headers(&self, headers: &mut HashMap<String, String>) {
        headers.insert("Content-Length".into(), self.len().to_string());
    }

    fn write_body(self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        Ok(stream.write_all(self)?)
    }
}

