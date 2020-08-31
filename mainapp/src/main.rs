#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::prelude::Utc;
use rocket::State;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

fn constant_str_print(s: String) {
    loop {
        let now = Utc::now();
        println!("{:?}: {}", now, s);
        sleep(Duration::from_secs(5));
    }
}

struct AppState {
    id: String,
}

#[get("/")]
fn get_id(state: State<AppState>) -> String {
    state.id.clone()
}

fn launch_web_server(id: String) {
    let state = AppState { id };

    rocket::ignite()
        .mount("/", routes![get_id])
        .manage(state)
        .launch();
}

fn main() {
    let id = Uuid::new_v4().to_string();
    let id_for_ws = id.clone();

    thread::spawn(move || {
        launch_web_server(id_for_ws);
    });

    constant_str_print(id);
}
