#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
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

struct PingState {
    count: Option<AtomicUsize>,
}

impl PingState {
    fn init(&mut self) {
        self.count = Some(AtomicUsize::new(0));
    }

    fn update_ping_count(&self, new: usize) {
        self.count.as_ref().unwrap().store(new, Ordering::Relaxed);
    }
}

fn read_ping_count(path: &Path) -> Result<usize, String> {
    let file_opts = OpenOptions::new().read(true).open(path);
    let file = match file_opts {
        Err(e) => {
            let e_msg = format!("WARNING: Couldn't open {}: {}", path.display(), e);
            return Err(e_msg);
        }
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    match reader.read_line(&mut line) {
        Err(e) => {
            let e_msg = format!("WARNING: Error reading {}: {}", path.display(), e);
            return Err(e_msg);
        }
        Ok(_) => (),
    }

    let new_count = match line.parse::<usize>() {
        Err(e) => {
            let e_msg = format!(
                "WARNING: Could not parse pingpong count from string: {}, {}",
                line, e
            );
            return Err(e_msg);
        }
        Ok(int) => int,
    };

    return Ok(new_count);
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

fn update_pings(file_path: String, state: Arc<PingState>) {
    let update_freq = Duration::from_secs(2);
    let path = Path::new(file_path.as_str());
    loop {
        let new_count = match read_ping_count(path) {
            Err(e) => {
                println!("{}", e);
                sleep(update_freq);
                continue;
            }
            Ok(count) => count,
        };

        state.update_ping_count(new_count);
        sleep(update_freq);
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
fn get_id(app_state: State<Arc<Mutex<AppState>>>, ping_state: State<Arc<PingState>>) -> String {
    let app_state = app_state.lock().unwrap();
    let id = &app_state.id.clone();
    let latest_input = &app_state.latest_input;
    let ping_count = &ping_state.count;

    let mut resp = format!("{} {}", latest_input, id);
    if ping_count.is_some() {
        let ping_count = ping_count.as_ref().unwrap();
        resp.push_str(format!("\nPing / Pongs: {:?}", ping_count).as_str());
    }
    resp
}

fn get_ts_file_path() -> String {
    let path = match env::var("MAINAPP_FILE_PATH") {
        Err(_) => {
            let test_file_path = String::from("test.txt");
            println!("Warning using test file path: {}", test_file_path);
            test_file_path
        }
        Ok(path) => path,
    };
    path
}

fn get_pingpong_file_path() -> Option<String> {
    let path = match env::var("PINGPONG_FILE_PATH") {
        Err(_) => {
            println!("WARNING! No PINGPONG_FILE_PATH defined, omitting ping pong results.");
            None
        }
        Ok(path) => Some(path),
    };
    path
}

fn main() {
    let ts_file_path = get_ts_file_path();
    let pingpong_file_path = get_pingpong_file_path();

    let (file_input_sender, file_input_receiver): (Sender<String>, Receiver<String>) = channel();

    let id = Uuid::new_v4().to_string();
    let id_for_state = id.clone();

    let default_msg = String::from("No input yet :(");

    let app_state = AppState {
        id: id_for_state,
        latest_input: default_msg,
    };

    let mut ping_state = PingState { count: None };
    if pingpong_file_path.is_some() {
        ping_state.init();
    }

    let app_state = Arc::new(Mutex::new(app_state));
    let state_for_update = app_state.clone();

    let ping_state = Arc::new(ping_state);
    let ping_state_for_update = ping_state.clone();

    if pingpong_file_path.is_some() {
        thread::spawn(move || {
            update_pings(pingpong_file_path.unwrap(), ping_state_for_update);
        });
    }

    thread::spawn(move || {
        read_and_send(ts_file_path, file_input_sender);
    });

    thread::spawn(move || {
        update_and_log(id, file_input_receiver, state_for_update);
    });

    let r = rocket::ignite()
        .mount("/", routes![get_id])
        .manage(app_state)
        .manage(ping_state);
    r.launch();
}
