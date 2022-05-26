extern crate dotenv;

use dotenv::dotenv;
use std::env;
use pingpong::config;
use pingpong::db_init::{establish_connection, init_ping_status, run_migrations};
use pingpong::server::build_web_server;

#[rocket::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_conn = establish_connection(db_url);
    run_migrations(&db_conn);

    let ping_id = config::get_ping_id();
    init_ping_status(&db_conn, ping_id);

    let server = build_web_server();
    let _ = server.launch().await.unwrap();
}

