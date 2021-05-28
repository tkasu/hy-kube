use crate::schema;
use crate::models;

use diesel::PgConnection;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;
use crate::models::PingStatus;


pub fn establish_connection(db_url: String) -> PgConnection {
    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

fn insert_new_ping(conn: &PgConnection, ping_id: String) {

    let new_ping_status = models::PingStatus {
        ping_id,
        ping_count: 0
    };

    diesel::insert_into(schema::ping_status::table)
        .values(new_ping_status)
        .execute(conn)
        .unwrap();
}


fn get_ping(conn: &PgConnection, app_ping_id: String) -> Option<PingStatus> {
    use schema::ping_status::dsl::*;
    use models::*;

    let ping = ping_status
        .filter(ping_id.eq(app_ping_id))
        .limit(1)
        .load::<PingStatus>(conn)
        .unwrap();

    match ping.get(0) {
        Some(ping) => Some((*ping).clone()),
        None => None
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