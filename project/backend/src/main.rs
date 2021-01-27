#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use bytes::Bytes;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::Duration;
use chrono::{DateTime, Utc};
use rocket_contrib::serve::StaticFiles;
use serde::{Deserialize, Serialize};
use std::env;
use std::process;
use std::borrow::Borrow;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread::{self, sleep};

const IMAGE_URL: &'static str = "https://picsum.photos/1200";
const IMAGE_CACHE_PATH: &'static str = "index_image.jpg";
const IMAGE_STATE_PATH: &'static str = "index_state.json";
const DEFAULT_IMAGE_UPDATE_INTERVAL_SECS: i64 = 60;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct ImageState {
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    last_update: DateTime<Utc>,
}

impl ImageState {
    fn open(file_path: String) -> Result<ImageState, Box<dyn Error>> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);
        let state = serde_json::from_reader(reader)?;
        Ok(state)
    }

    fn save(&self, file_path: String) -> Result<usize, std::io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file_path)
            .unwrap();

        let json = serde_json::to_string(&self).unwrap();
        let save_res = file.write(json.as_bytes());
        save_res
    }

    fn next_update(self, dur: Duration) -> DateTime<Utc> {
        let next_update = self.last_update.checked_add_signed(dur).unwrap();
        next_update
    }
}

fn fetch_image(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;
    let image = resp.bytes().unwrap();
    Ok(image)
}

fn fetch_state() -> Result<ImageState, Box<dyn std::error::Error>> {
    let state_cache = ImageState::open(String::from(IMAGE_STATE_PATH));
    let state = match state_cache {
        Err(e) => {
            println!("WARNING: Could not read existing image state: {:?}", e);
            let state = ImageState {
                last_update: Utc::now(),
            };
            let save_res = state.save(String::from(IMAGE_STATE_PATH));
            match save_res {
                Ok(_) => (),
                Err(e) => {
                    println!("WARNING: Could save the state file: {:?}", e);
                    return Err(e.into());
                }
            }
            state
        }
        Ok(state) => state,
    };
    Ok(state)
}

fn wait_until_update(state: &ImageState, image_update_interval: i64) {
    let next_update = state.next_update(Duration::seconds(image_update_interval));

    let current_time = Utc::now();
    let dur_until_update = next_update - current_time;

    if dur_until_update <= Duration::milliseconds(0) {
        println!("Image update interval reached!");
    } else {
        println!("Postponing image update until {}", next_update);
        sleep(std::time::Duration::from_millis(
            dur_until_update.num_milliseconds() as u64,
        ));
    }
}

fn update_image() -> Result<(), Box<dyn std::error::Error>> {
    let img = fetch_image(IMAGE_URL)?;

    // Update cache image
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(IMAGE_CACHE_PATH)
        .unwrap();
    file.write(img.borrow()).unwrap();

    // Move cached image to to static
    std::fs::copy(IMAGE_CACHE_PATH, "static/daily_pic.jpg")?;

    // Update state file
    let state = ImageState {
        last_update: Utc::now(),
    };
    state.save(String::from(IMAGE_STATE_PATH))?;

    Ok(())
}

fn update_image_loop() {
    loop {
        let image_update_interval: i64 = match env::var("IMAGE_UPDATE_INTERVAL") {
            Err(_) => {
                let update_interval = DEFAULT_IMAGE_UPDATE_INTERVAL_SECS;
                println!("Using default image update interval: {} seconds", update_interval);
                update_interval
            }
            Ok(update_interval) => {
                let update_interval: i64 = update_interval.parse().unwrap_or_else(|e| {
                    println!("Failed to parse IMAGE_UPDATE_INTERVAL `{}`: {}", update_interval, e);
                    process::exit(1);
                });
                println!("Using image update interval: {} seconds", update_interval);
                update_interval
            },
        };

        let state = fetch_state();
        match state {
            Ok(state) => {
                wait_until_update(&state, image_update_interval);
                println!("Updating image!");
                let update_res = update_image();
                match update_res {
                    Ok(_) => (),
                    Err(e) => {
                        println!("WARNING!, Failed to update image, {:?}", e);
                        println!("Retrying in 60 seconds.");
                        sleep(std::time::Duration::from_secs(60));
                    }
                }
            }
            Err(e) => {
                println!("WARNING!, Failed to save the state file, {:?}", e);
                println!("Retrying in 60 seconds.");
                sleep(std::time::Duration::from_secs(60));
            }
        }
    }
}

fn main() {
    thread::spawn(move || update_image_loop());

    rocket::ignite()
        .mount("/", StaticFiles::from("./static"))
        .launch();
}
