use chrono::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();

    loop {
        let now = Utc::now();
        println!("{:?}: {}", now, id);
        sleep(Duration::new(5, 0));
    }
}
