use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use log::info;
use serde_json::json;
use crate::log_panic;

pub struct HttpServer {
    listen_addr: String,
}

pub trait HttpServerTrt {
    fn create_addr(addr: String) -> HttpServer;
    fn create_port(port: u32) -> HttpServer;
    fn start_blocking(&self);
}

impl HttpServerTrt for HttpServer {
    fn create_addr(listen_addr: String) -> HttpServer {
        HttpServer { listen_addr }
    }

    fn create_port(port: u32) -> HttpServer {
        if port > 65535 {
            log_panic!("Port cannot be higher than 65535, was: {port}")
        }
        let addr = format!("0.0.0.0:{port}");
        info!("Starting HTTP server on: {addr}");
        HttpServer { listen_addr: addr }
    }

    fn start_blocking(&self) {
        let listener = TcpListener::bind(&self.listen_addr).unwrap_or_else(|e| {
            log_panic!("Could not start listening on {addr}, reason:\n{reason}", addr = self.listen_addr, reason = e.to_string())
        });

        for stream in listener.incoming() {
            let stream = stream.unwrap_or_else(|e| {
                log_panic!("Could not open tcp stream, reason:\n{}", e.to_string());
            });

            handle_conn(stream, HashSet::from([
                ConnHandler {
                    path: "/",
                    method: "GET",
                    handler_func: || Ok(Response { status_code: 200, response_body: json!({"status": "ok"}).to_string() }),
                }]));
        }
    }
}

fn handle_conn(mut stream: TcpStream, handlers: HashSet<ConnHandler>) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let first_line: Vec<&str> = http_request[0].split(" ").collect();
    let method = first_line[0];
    let path = first_line[1];
    let _protocol = first_line[2];
    let _headers = &http_request[1..];

    match handlers.iter().find(|&handler| handler.compare_endpoint(method, path)) {
        None => {
            let status_line = "HTTP/1.1 404 NOT_FOUND";
            let contents = format!("Resource: {path} not found.");
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap()
        }
        Some(handler) => {
            let res = (handler.handler_func)().unwrap();
            let status_line = format!("HTTP/1.1 {} OK", res.status_code);
            let contents = res.response_body;
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap()
        }
    }
}

pub struct ConnHandler {
    method: &'static str,
    path: &'static str,
    handler_func: fn() -> Result<Response, String>,
}

impl PartialEq for ConnHandler {
    fn eq(&self, other: &Self) -> bool {
        self.path.to_lowercase() == other.path.to_lowercase() &&
            self.method.to_lowercase() == other.method.to_lowercase()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Hash for ConnHandler {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.method.hash(state);
    }
}

impl Eq for ConnHandler {}

impl ConnHandler {
    fn compare_endpoint(&self, method: &str, path: &str) -> bool {
        self.path.to_lowercase() == path.to_lowercase() &&
            self.method.to_lowercase() == method.to_lowercase()
    }
}

pub struct Response {
    status_code: u16,
    response_body: String,
}