#[macro_use]
extern crate rocket;

use reqwest;
use rocket::{Build, Rocket, State};
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub struct AppState {
    id: String,
    latest_input: String,
    message: String,
}

impl AppState {
    pub fn new(id: String, message: String) -> Self {
        let default_msg = String::from("No input yet :(");

        AppState {
            id,
            message,
            latest_input: default_msg,
        }
    }

    fn update_latest_input(&mut self, new: String) {
        self.latest_input = new;
    }
}

#[derive(Deserialize)]
struct PingsApiResponse {
    ping_count: usize,
}

pub struct PingState {
    count: Option<AtomicUsize>,
}

impl PingState {
    pub fn new() -> Self {
        Self {
            count: Some(AtomicUsize::new(0)),
        }
    }

    fn update_ping_state(&self, api_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let api_response = Self::read_ping_count(api_url)?;
        self.update_ping_count(api_response.ping_count);
        Ok(())
    }

    fn update_ping_count(&self, new: usize) {
        self.count.as_ref().unwrap().store(new, Ordering::Relaxed);
    }

    fn read_ping_count(api_url: &str) -> Result<PingsApiResponse, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(api_url)?;
        let ping_status = resp.json::<PingsApiResponse>()?;
        Ok(ping_status)
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

pub fn read_and_send(file_path: String, sender: Sender<String>) {
    let path = Path::new(file_path.as_str());

    let mut line_counter = 0;
    loop {
        line_counter = read_lines_from(path, &sender, line_counter);
        sleep(Duration::from_secs(5));
    }
}

pub fn update_pings(api_url: String, state: Arc<PingState>) {
    let api_url = api_url.as_str();
    let update_freq = Duration::from_secs(2);
    loop {
        state.update_ping_state(api_url).unwrap_or_else(|err| {
            println!("WARNING: Problem updating ping count: {}", err);
        });
        sleep(update_freq);
    }
}

pub fn update_and_log(s: String, receiver: Receiver<String>, state: Arc<Mutex<AppState>>) {
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
fn get_default(
    app_state: &State<Arc<Mutex<AppState>>>,
    ping_state: &State<Arc<PingState>>,
) -> String {
    let app_state = app_state.lock().unwrap();
    let id = &app_state.id.clone();
    let message = &app_state.message.clone();
    let latest_input = &app_state.latest_input;
    let ping_count = &ping_state.count;

    let mut resp = format!("{}\n{} {}", message, latest_input, id);
    if ping_count.is_some() {
        let ping_count = ping_count.as_ref().unwrap();
        resp.push_str(format!("\nPing / Pongs: {:?}", ping_count).as_str());
    }
    resp
}

pub fn build_web_server(
    app_state: Arc<Mutex<AppState>>,
    ping_state: Arc<PingState>,
) -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_default])
        .manage(app_state)
        .manage(ping_state)
}
