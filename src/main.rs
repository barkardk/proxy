mod server;
mod http;

use server::Server;
use tracing::info;
use tracing_subscriber;


fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

