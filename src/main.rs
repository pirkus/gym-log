mod server;

use std::env;
use log::info;
use crate::server::{HttpServer, HttpServerTrt};

fn main() {
    env_logger::init();
    let port = env::var("HTTP_PORT").unwrap_or_else(|_| {
        info!("No HTTP_PORT env var set, defaulting to 8090");
        String::from("8090")
    });

    HttpServer::start(port);
}