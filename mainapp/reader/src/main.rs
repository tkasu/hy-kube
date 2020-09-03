#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

struct AppState {
    id: String,
    latest_input: String,
}

impl AppState {
    fn update_latest_input(&mut self, new: String) {
        self.latest_input = new;
    }
}

fn read_lines_from(path: &Path, sender: &Sender<String>, from_line: usize) -> usize {
    let mut line_counter = from_line;

    let file_opts = OpenOptions::new().read(true).open(path);
    let file = match file_opts {
        Err(e) => {
            println!("WARNING: Couldn't open {}: {}", path.display(), e);
            return line_counter;
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    for line in reader.lines().skip(from_line) {
        line_counter += 1;
        let line = line.unwrap();
        sender.send(line).unwrap();
    }
    line_counter
}

fn read_and_send(file_path: String, sender: Sender<String>) {
    let path = Path::new(file_path.as_str());

    let mut line_counter = 0;
    loop {
        line_counter = read_lines_from(path, &sender, line_counter);
        sleep(Duration::from_secs(5));
    }
}

fn update_and_log(s: String, receiver: Receiver<String>, state: Arc<Mutex<AppState>>) {
    loop {
        match receiver.recv_timeout(Duration::from_secs(10)) {
            Err(_) => println!("WARNING! No new data received in 10 seconds!"),
            Ok(input) => {
                let to_print = input.clone();
                let mut guard = state.lock().unwrap();
                guard.update_latest_input(input);
                println!("{}: {}", to_print, s);
            }
        };
    }
}

#[get("/")]
fn get_id(state: State<Arc<Mutex<AppState>>>) -> String {
    let app_state = state.lock().unwrap();
    let id = &app_state.id.clone();
    let latest_input = &app_state.latest_input;

    format!("{} {}", latest_input, id)
}

fn get_file_path(env: &str) -> String {
    let s = match env::var(env) {
        Err(_) => {
            let test_file_path = String::from("test.txt");
            println!("Warning using test file path: {}", test_file_path);
            test_file_path
        }
        Ok(path) => path,
    };
    s
}

fn main() {
    let env_var_name: &'static str = "MAINAPP_FILE_PATH";
    let file_path = get_file_path(env_var_name);

    let (file_input_sender, file_input_receiver): (Sender<String>, Receiver<String>) = channel();

    let id = Uuid::new_v4().to_string();
    let id_for_state = id.clone();

    let default_msg = String::from("No input yet :(");

    let state = AppState {
        id: id_for_state,
        latest_input: default_msg,
    };
    let state = Arc::new(Mutex::new(state));
    let state_for_update = state.clone();

    thread::spawn(move || {
        read_and_send(file_path, file_input_sender);
    });

    thread::spawn(move || {
        update_and_log(id, file_input_receiver, state_for_update);
    });

    let r = rocket::ignite().mount("/", routes![get_id]).manage(state);
    r.launch();
}
