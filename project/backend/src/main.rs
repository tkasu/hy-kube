#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use backend::start_web_server;
use backend::update_image_loop;
use backend::Config;
use std::process;
use std::thread;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Using config: {:?}", config);

    thread::spawn(move || update_image_loop(config));
    start_web_server();
}
