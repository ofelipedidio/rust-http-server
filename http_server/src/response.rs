use std::{collections::HashMap, net::TcpStream, io::Write};

pub trait HttpBody {
    fn write_headers(&self, headers: &mut HashMap<String, String>);

    fn write_body(&self, stream: &mut TcpStream);
}

impl HttpBody for String {
    fn write_headers(&self, headers: &mut HashMap<String, String>) {
        let len = self.bytes().len();
        headers.insert("Content-Length".to_string(), len.to_string());
    }

    fn write_body(&self, stream: &mut TcpStream) {
        stream.write_all(self.as_bytes()).unwrap();
    }
}

// impl HttpBody for String {
//     fn write_headers(&self, headers: &mut HashMap<String, String>) {
//         headers.insert("Transfer-Encoding".to_string(), "chunked".to_string());
//     }
// 
//     fn write_body(&self, stream: &mut TcpStream) {
//         ...
//     }
// }

impl <B: HttpBody> HttpBody for Box<B> {
    fn write_body(&self, stream: &mut TcpStream) {
        self.as_ref().write_body(stream);
    }

    fn write_headers(&self, headers: &mut HashMap<String, String>) {
        self.as_ref().write_headers(headers);
    }
}

impl HttpBody for Box<dyn HttpBody> {
    fn write_body(&self, stream: &mut TcpStream) {
        self.as_ref().write_body(stream);
    }

    fn write_headers(&self, headers: &mut HashMap<String, String>) {
        self.as_ref().write_headers(headers);
    }
}

pub enum HttpVersion {
    Http1_1,
}

impl HttpVersion {
    pub fn to_string(&self) -> String {
        match self {
            HttpVersion::Http1_1 => "HTTP/1.1",
        }.into()
    }
}

pub enum Status {
    Ok,
    NotFound,
    Custom(usize, String),
}

impl Status {
    pub fn code(&self) -> usize {
        match self {
            Status::Ok => 200,
            Status::NotFound => 404,
            Status::Custom(code, _) => code.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Status::Ok => "Ok",
            Status::NotFound => "Not Found",
            Status::Custom(_, message) => message,
        }.into()
    }
}

pub struct HttpResponse<B: HttpBody> {
    http_version: HttpVersion,
    status: Status,
    headers: HashMap<String, String>,
    body: B,
}

impl <B: HttpBody> HttpResponse<B> {
    pub fn new(http_version: HttpVersion, status: Status, headers: HashMap<String, String>, body: B) -> Self {
        HttpResponse { http_version, status, headers, body }
    }

    pub fn write(&mut self, stream: &mut TcpStream) {
        let status_line = format!("{} {} {}\r\n", self.http_version.to_string(), &self.status.code(), &self.status.message());
        stream.write_all(status_line.as_bytes()).unwrap();

        self.body.write_headers(&mut self.headers);
        for (key, value) in self.headers.iter() {
            stream.write_all(format!("{}: {}\r\n", key, value).as_bytes()).unwrap();
        }
        stream.write_all("\r\n".as_bytes()).unwrap();
        
        self.body.write_body(stream);
    }
}
