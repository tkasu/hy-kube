extern crate dotenv;

use dotenv::dotenv;
use reader::{build_web_server, read_and_send, update_and_log, update_pings, AppState, PingState};
use std::env;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

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

#[rocket::main]
async fn main() {
    dotenv().ok();

    let ts_file_path = get_ts_file_path();

    let pingpong_app_url =
        env::var("PINGPONG_APP_URL").unwrap_or("http://localhost:5000".to_string());
    let pings_endpoint_location = "/pings";
    let pingpong_endpoint_url = format!("{}{}", pingpong_app_url, pings_endpoint_location);
    println!("Using pingpong api url: {}", pingpong_endpoint_url);

    let id = Uuid::new_v4().to_string();

    let message = env::var("MESSAGE").unwrap_or("Generic default message".to_string());

    let id_for_state = id.clone();
    let app_state = AppState::new(id_for_state, message);
    let app_state = Arc::new(Mutex::new(app_state));
    let state_for_update = app_state.clone();

    let ping_state = PingState::new(pingpong_endpoint_url);
    let ping_state = Arc::new(ping_state);
    let ping_state_for_update = ping_state.clone();

    let (file_input_sender, file_input_receiver): (Sender<String>, Receiver<String>) = channel();

    thread::spawn(move || {
        update_pings(ping_state_for_update);
    });

    thread::spawn(move || {
        read_and_send(ts_file_path, file_input_sender);
    });

    thread::spawn(move || {
        update_and_log(id, file_input_receiver, state_for_update);
    });

    let server = build_web_server(app_state, ping_state);
    let _ = server.launch().await.unwrap();
}
