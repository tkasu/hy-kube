use crate::db::{self, PingPongDbConn};
use crate::models;

use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use rocket_db_pools::Database;

#[get("/pingpong")]
async fn pong(db: &PingPongDbConn) -> String {
    let old_count = db::get_ping(db).await.unwrap();
    db::inc_ping(db).await;
    format!("pong {}", old_count.ping_count)
}

#[get("/pingpong/pings")]
async fn pings(db: &PingPongDbConn) -> Json<models::PingStatus> {
    let ping_status = db::get_ping(db).await.unwrap();
    Json(ping_status)
}

pub fn build_web_server() -> Rocket<Build> {
    let conn = PingPongDbConn::init();

    rocket::build()
        .attach(conn)
        .attach(AdHoc::try_on_ignite(
            "DB Migrations and state init.",
            db::run_migrations_and_init_state,
        ))
        .mount("/", routes![pong, pings])
}
