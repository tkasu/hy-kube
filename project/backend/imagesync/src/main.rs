extern crate rocket;

use imagesync::config::Config;
use imagesync::image::update_image_loop;
use imagesync::server::build_web_server;
use std::process;
use std::thread;

#[rocket::main]
async fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Using config: {:?}", config);

    thread::spawn(move || update_image_loop(config));

    let server = build_web_server();
    let _ = server.launch().await.unwrap();
}
