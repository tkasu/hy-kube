#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

struct AppState {
    ping_count: Arc<Mutex<usize>>,
    file_path: String,
}

impl AppState {
    fn get_pings(&self) -> usize {
        let count = self.ping_count.lock().unwrap();
        *count
    }

    fn inc_pings(&self) {
        let mut guard = self.ping_count.lock().unwrap();
        let new_count = *guard + 1;
        let path = Path::new(self.file_path.as_str());
        rewrite_to_file(String::from(new_count.to_string()), path);
        *guard = new_count;
    }
}

fn rewrite_to_file(s: String, path: &Path) {
    let file_opts = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path);

    let mut file = match file_opts {
        Err(e) => panic!("Couldn't open {}: {}", path.display(), e),
        Ok(file) => file,
    };

    match file.write(s.as_bytes()) {
        Err(e) => panic!("Couldn't write to {}: {}", path.display(), e),
        Ok(_) => (),
    };
}

fn init_file(path: &Path) {
    let content = String::from("0");
    rewrite_to_file(content, path)
}

#[get("/")]
fn pong(state: State<AppState>) -> String {
    let old_count = state.get_pings();
    state.inc_pings();
    format!("pong {}", old_count)
}

fn main() {
    let file_path = match env::var("PINGPONG_FILE_PATH") {
        Err(_) => {
            let test_file_path = String::from("pingpong.txt");
            println!("Warning using test file path: {}", test_file_path);
            test_file_path
        }
        Ok(path) => path,
    };

    init_file(Path::new(&file_path));

    let state = AppState {
        ping_count: Arc::new(Mutex::new(0)),
        file_path,
    };

    rocket::ignite()
        .manage(state)
        .mount("/", routes![pong])
        .launch();
}
