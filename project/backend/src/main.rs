#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use serde::Serialize;
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct AppState {
    name: String,
}

#[get("/")]
fn index(state: State<AppState>) -> Template {
    let context = state.inner();
    Template::render("index", &context)
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    let mut context = HashMap::new();
    context.insert("name", name);
    Template::render("index", &context)
}

fn main() {
    let state = AppState { name: String::from("Kube") };

    rocket::ignite()
        .attach(Template::fairing())
        .manage(state)
        .mount("/", routes![index, hello])
        .launch();
}