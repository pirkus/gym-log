mod server;
mod macros;

use crate::server::http_server::{HttpServer, HttpServerTrt};

fn main() {
    env_logger::init();

    HttpServer::create_port(8090)
        .start_blocking();
}