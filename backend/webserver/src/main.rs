use crate::webserver::listen;
use crate::database_server::connect;

mod webserver;
mod database_server;

#[tokio::main]
async fn main() {
    listen();
    connect();
    let (first, second) =tokio::join!(listen(), connect());
}
