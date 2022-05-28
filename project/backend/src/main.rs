extern crate rocket;

use backend::config::Config;
use backend::db;
use backend::image::update_image_loop;
use backend::server::build_web_server;
use std::process;
use std::thread;

#[rocket::main]
async fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Using config: {:?}", config);

    let db_conn = db::establish_connection(config.db_url.as_str()).await;
    db::run_migrations(&db_conn).await;

    thread::spawn(move || update_image_loop(config));

    let server = build_web_server();
    let _ = server.launch().await.unwrap();
}
