use crate::db_live::{self, PingPongDbConn};
use crate::models;

use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use rocket_db_pools::Database;


#[get("/")]
async fn pong(db: &PingPongDbConn) -> String {
    let old_count = db_live::get_ping(db).await.unwrap();
    db_live::inc_ping(db).await;
    format!("pong {}", old_count.ping_count)
}

#[get("/pings")]
async fn pings(db: &PingPongDbConn) -> Json<models::PingStatus> {
    let ping_status = db_live::get_ping(db).await.unwrap();
    Json(ping_status)
}

pub fn build_web_server() -> Rocket<Build> {
    let conn = PingPongDbConn::init();

    rocket::build()
        .attach(conn)
        .mount("/", routes![pong, pings])
}
