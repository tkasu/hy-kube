#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;

struct AppState {
    ping_count: AtomicUsize,
}

impl AppState {
    fn get_pings(&self) -> usize {
        let count = self.ping_count.load(Ordering::Relaxed);
        count
    }

    fn inc_pings(&self) {
        self.ping_count.fetch_add(1, Ordering::Relaxed);
    }
}

#[get("/")]
fn pong(state: State<AppState>) -> String {
    let old_count = state.get_pings();
    state.inc_pings();
    format!("pong {}", old_count)
}

fn main() {
    let state = AppState { ping_count: AtomicUsize::new(0) };

    rocket::ignite()
        .manage(state)
        .mount("/", routes![pong])
        .launch();
}