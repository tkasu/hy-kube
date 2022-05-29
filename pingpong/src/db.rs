use crate::{config, models};
use rocket::{fairing, Build, Rocket};

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

pub async fn init_ping_status(db: &PingPongDbConn) {
    let ping_fetch_res = get_ping(&db).await;
    match ping_fetch_res {
        Ok(ping) => println!("Found existing ping status: {:?}", ping),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                println!("Could not found existing ping status, new one will be created.");
                add_ping(&db).await;
            }
            err => panic!("Could not fetch ping status from database {:?}", err),
        },
    }
}

pub async fn run_migrations_and_init_state(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = PingPongDbConn::fetch(&rocket) {
        sqlx::migrate!().run(&db.0).await.unwrap();
        init_ping_status(&db).await;
        Ok(rocket)
    } else {
        Err(rocket)
    }
}

async fn add_ping(db: &PingPongDbConn) {
    let ping_id = config::get_ping_id();

    sqlx::query(
        "INSERT INTO ping_status (ping_id, ping_count) \
        VALUES ($1, 0)",
    )
    .bind(ping_id)
    .execute(&db.0)
    .await
    .unwrap();
}
