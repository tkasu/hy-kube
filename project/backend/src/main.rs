use chrono::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

fn main() {
    loop {
        let now = Utc::now();
        let id = Uuid::new_v4();

        println!("{:?}: {}", now, id);
        sleep(Duration::new(5, 0));
    }
}
