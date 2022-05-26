/*
 Module used when initing the db connection, uses diesel.
 This is technical debt, ideally should be in sync with db_init
 */
use crate::models;
use crate::schema;

use crate::models::PingStatus;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel_migrations::embed_migrations;

embed_migrations!();

pub fn establish_connection(db_url: String) -> PgConnection {
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn run_migrations(conn: &PgConnection) {
    embedded_migrations::run(conn).unwrap();
}

fn insert_new_ping(conn: &PgConnection, ping_id: String) {
    let new_ping_status = models::PingStatus {
        ping_id,
        ping_count: 0,
    };

    diesel::insert_into(schema::ping_status::table)
        .values(new_ping_status)
        .execute(conn)
        .unwrap();
}

pub fn get_ping(conn: &PgConnection, app_ping_id: String) -> Option<PingStatus> {
    use models::*;
    use schema::ping_status::dsl::*;

    let ping = ping_status
        .filter(ping_id.eq(app_ping_id))
        .limit(1)
        .load::<PingStatus>(conn)
        .unwrap();

    match ping.get(0) {
        Some(ping) => Some((*ping).clone()),
        None => None,
    }
}

pub fn init_ping_status(conn: &PgConnection, ping_id: String) {
    let maybe_ping = get_ping(conn, ping_id.clone());
    if let Some(ping) = maybe_ping {
        println!("Found existing ping status: {:?}", ping)
    } else {
        println!("Could not found existing ping status, new one will be created.");
        insert_new_ping(conn, ping_id);
    }
}
