use tokio;

mod server;

use server::{chat_server, http_server};

#[tokio::main]
async fn main() {
    http_server::listen();
    chat_server::launch().await;
}
