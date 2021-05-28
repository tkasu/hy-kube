use crate::models;

use rocket::{State, Rocket, Build};
use rocket_contrib::json::Json;
use std::sync::{Arc, Mutex};

use rocket_contrib::databases::diesel;


#[database("pingpongdb")]
struct PingPongDbConn(diesel::PgConnection);

/*
pub struct AppState {
    ping_count: Arc<Mutex<i32>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            ping_count: Arc::new(Mutex::new(0)),
        }
    }

    fn get_pings(&self) -> i32 {
        let count = self.ping_count.lock().unwrap();
        *count
    }

    fn inc_pings(&self) {
        let mut guard = self.ping_count.lock().unwrap();
        let new_count = *guard + 1;
        *guard = new_count;
    }
}
 */

#[get("/")]
fn pong(state: State<AppState>) -> String {
    let old_count = state.get_pings();
    state.inc_pings();
    format!("pong {}", old_count)
}

#[get("/pings")]
fn pings(state: State<AppState>) -> Json<models::PingStatus> {
    let pings = state.get_pings();
    Json(models::PingStatus { ping_id: String::from("pingpong"), ping_count: pings })
}

pub fn build_web_server(state: AppState) -> Rocket<Build> {
    let conn = PingPongDbConn::fairing();

    rocket::build()
        .attach(conn)
        .manage(state)
        .mount("/", routes![pong, pings])
}
