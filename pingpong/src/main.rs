extern crate dotenv;

use dotenv::dotenv;
use std::env;
use pingpong::db::{establish_connection, init_ping_status};
use pingpong::server::{build_web_server, AppState};

#[rocket::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_conn = establish_connection(db_url);
    init_ping_status(&db_conn, String::from("pingpong_default"));

    let state = AppState::new();
    let server = build_web_server(state);

    server.launch().await;
}

