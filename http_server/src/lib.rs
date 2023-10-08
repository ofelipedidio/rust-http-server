pub mod path;
pub mod request;
pub mod response;
pub mod query;
pub mod router;

use crate::router::RequestHandler;

use std::{
    net::{TcpListener, ToSocketAddrs, TcpStream}, 
    io::{prelude::*, BufReader}, collections::HashMap};

use path::Path;
use query::QueryPath;
use request::HttpRequest;
use response::{HttpResponse, HttpBody};

fn handle_request(request: HttpRequest<QueryPath<String>>) -> HttpResponse<Box<dyn HttpBody>> {
    //let mut router = Router::new("./server");

    /*
    let default_name = "(use name query parameter)".to_string();
    let name = request.path.query.get(&"name".to_string()).unwrap_or(&default_name);

    let body = match request.path.path.as_str() {
        "/" => format!("Hello, {}", &name),
        _ => "Good morning".to_string(),
    }.to_string();

    HttpResponse::new(response::HttpVersion::Http1_1, Status::Ok, HashMap::new(), body)
    */

    // router.handle_request(request)
    HttpResponse::new(response::HttpVersion::Http1_1, response::Status::Ok, HashMap::new(), Box::new("asd".to_string()))
}

fn handle_stream<P: Path, B: HttpBody>(mut stream: TcpStream, request_handler: RequestHandler<P, B>) {
    let reader = BufReader::new(&mut stream);
    let mut lines = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty());

    let status_line = lines.next().unwrap();
    let header_lines: Vec<_> = lines.collect();
    let request = HttpRequest::parse_request(status_line, header_lines);
    println!("{:#?}", &request);

    let mut response = request_handler(request);

    response.write(&mut stream);
}

pub fn run_server<A: ToSocketAddrs>(addr: A) -> ! {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream, handle_request);
    }
    panic!("Listener closed");
}

