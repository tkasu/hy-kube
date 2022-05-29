extern crate dotenv;

use dotenv::dotenv;
use pingpong::server::build_web_server;

#[rocket::main]
async fn main() {
    dotenv().ok();

    let server = build_web_server();
    let _ = server.launch().await.unwrap();
}
