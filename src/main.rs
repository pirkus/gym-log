mod server;
mod macros;

use server::conn_handler::HttpServerTrt;
use crate::server::http_server::HttpServer;

fn main() {
    env_logger::init();

    HttpServer::create_port(8090)
        .start_blocking();
}