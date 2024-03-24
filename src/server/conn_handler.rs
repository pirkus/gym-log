use std::hash::{Hash, Hasher};
use crate::server::http_server::{HttpServer, Response};

pub trait HttpServerTrt {
    fn create_addr(addr: String) -> HttpServer;
    fn create_port(port: u32) -> HttpServer;
    fn start_blocking(&self);
}

pub struct ConnHandler {
    method: String,
    path: String,
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
    pub fn compare_endpoint(&self, method: &str, path: &str) -> bool {
        self.path.to_lowercase() == path.to_lowercase() &&
            self.method.to_lowercase() == method.to_lowercase()
    }

    pub fn handle(&self) -> Result<Response, String> {
        (self.handler_func)()
    }

    pub fn new(path: &str, method: &str, handler_func: fn() -> Result<Response, String>) -> ConnHandler {
        ConnHandler { path: path.to_string(), method: method.to_string(), handler_func }
    }
}
