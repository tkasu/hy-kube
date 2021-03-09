extern crate dotenv;

use dotenv::dotenv;
use pingpong::start_web_server;
use pingpong::AppState;

fn main() {
    dotenv().ok();

    let state = AppState::new();
    start_web_server(state)
}
