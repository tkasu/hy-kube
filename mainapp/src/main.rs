#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::prelude::Utc;
use rocket::handler::{Handler, Outcome};
use rocket::http::Method;
use rocket::{Data, Request, Route};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

fn constant_str_print(s: String) {
    loop {
        let now = Utc::now();
        println!("{:?}: {}", now, s);
        sleep(Duration::new(5, 0));
    }
}

#[derive(Clone)]
struct IdHandler {
    id: String,
}

impl Handler for IdHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        let ret_val = self.id.clone();
        Outcome::from(req, ret_val)
    }
}

impl Into<Vec<Route>> for IdHandler {
    fn into(self) -> Vec<Route> {
        vec![Route::new(Method::Get, "/", self)]
    }
}

fn launch_web_server(id: String) {
    let handler = IdHandler { id };
    rocket::ignite().mount("/", handler).launch();
}

fn main() {
    let id = Uuid::new_v4().to_string();
    let id_for_ws = id.clone();

    thread::spawn(move || {
        launch_web_server(id_for_ws);
    });

    constant_str_print(id);
}
