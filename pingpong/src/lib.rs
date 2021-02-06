#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::{Arc, Mutex};

pub struct AppState {
    ping_count: Arc<Mutex<usize>>,
}

#[derive(Serialize)]
struct PingStatus {
    ping_count: usize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            ping_count: Arc::new(Mutex::new(0)),
        }
    }

    fn get_pings(&self) -> usize {
        let count = self.ping_count.lock().unwrap();
        *count
    }

    fn inc_pings(&self) {
        let mut guard = self.ping_count.lock().unwrap();
        let new_count = *guard + 1;
        *guard = new_count;
    }
}

#[get("/")]
fn pong(state: State<AppState>) -> String {
    let old_count = state.get_pings();
    state.inc_pings();
    format!("pong {}", old_count)
}

#[get("/pings")]
fn pings(state: State<AppState>) -> Json<PingStatus> {
    let pings = state.get_pings();
    Json(PingStatus { ping_count: pings })
}

pub fn start_web_server(state: AppState) {
    rocket::ignite()
        .manage(state)
        .mount("/", routes![pong, pings])
        .launch();
}
