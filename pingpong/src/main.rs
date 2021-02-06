use pingpong::start_web_server;
use pingpong::AppState;

fn main() {
    let state = AppState::new();
    start_web_server(state)
}
