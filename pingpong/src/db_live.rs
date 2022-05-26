/*
Module used within api for db connection, uses sqlx.
*/
use crate::{config, models};

use crate::models::PingStatus;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("pingpongdb")]
pub struct PingPongDbConn(sqlx::PgPool);

pub async fn get_ping(db: &PingPongDbConn) -> Result<PingStatus, sqlx::Error> {
    let ping_id = config::get_ping_id();

    sqlx::query_as::<_, models::PingStatus>(
        "SELECT * \
    FROM ping_status \
    WHERE ping_id = $1",
    )
    .bind(ping_id)
    .fetch_one(&db.0)
    .await
}

pub async fn inc_ping(db: &PingPongDbConn) {
    let ping_id = config::get_ping_id();

    sqlx::query(
        "UPDATE ping_status \
        SET ping_count = ping_count + 1 \
        WHERE ping_id = $1",
    )
    .bind(ping_id)
    .execute(&db.0)
    .await
    .unwrap();
}
