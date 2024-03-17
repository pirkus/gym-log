use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use log::error;

pub struct HttpServer {
    port: String,
    listener: TcpListener
}

pub trait HttpServerTrt {
    fn start(addr: String) -> HttpServer;
}

impl HttpServerTrt for HttpServer {
    fn start(port: String) -> HttpServer {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap_or_else(|e| {
            let msg = format!("Could not start listening on port {}, reason:\n{}", port, e.to_string());
            error!("{}" ,msg);
            panic!("{}", msg)
        });

        for stream in listener.incoming() {
            let stream = stream.unwrap_or_else(|e| {
                let msg = format!("Could not open tcp stream, reason:\n{}", e.to_string());
                error!("{}" ,msg);
                panic!("{}", msg)
            });

            handle_conn(stream, Vec::new());
        }

        HttpServer { port, listener }
    }
}

fn handle_conn(mut stream: TcpStream, handlers: Vec<ConnHandler>) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = "{\"status\": \"ok\"}";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

pub struct ConnHandler {
    method: String,
    path: String,
    handler_func: fn() -> Result<String, String>
}