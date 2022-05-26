use std::env;

pub fn get_ping_id() -> String {
    env::var("PING_ID").unwrap_or(String::from("pingpong_default"))
}
