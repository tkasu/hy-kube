#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use bytes::Bytes;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::Duration;
use chrono::{DateTime, Utc};
use rocket::response::NamedFile;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread::sleep;

const IMAGE_URL: &'static str = "https://picsum.photos/1200";
const DEFAULT_IMAGE_CACHE_PATH: &'static str = "index_image.jpg";
const DEFAULT_IMAGE_STATE_PATH: &'static str = "index_state.json";
const DEFAULT_IMAGE_UPDATE_INTERVAL_SECS: i64 = 60;

#[derive(Clone, Debug)]
pub struct Config {
    image_update_interval: i64,
    image_cache_path: String,
    image_state_path: String,
}

impl Config {
    fn get_update_interval() -> Result<i64, String> {
        let image_update_interval: Result<i64, String> = match env::var("IMAGE_UPDATE_INTERVAL") {
            Err(_) => {
                let update_interval = DEFAULT_IMAGE_UPDATE_INTERVAL_SECS;
                Ok(update_interval)
            }
            Ok(update_interval) => {
                let update_interval = match update_interval.parse::<i64>() {
                    Ok(interval) => Ok(interval),
                    Err(e) => {
                        let err_str = format!(
                            "Failed to parse IMAGE_UPDATE_INTERVAL `{}`: {}",
                            update_interval, e
                        );
                        Err(err_str)
                    }
                };
                update_interval
            }
        };

        image_update_interval
    }

    fn get_image_state_path() -> String {
        env::var("IMAGE_STATE_PATH").unwrap_or(DEFAULT_IMAGE_STATE_PATH.to_string())
    }

    fn get_image_cache_path() -> String {
        env::var("IMAGE_CACHE_PATH").unwrap_or(DEFAULT_IMAGE_CACHE_PATH.to_string())
    }

    pub fn new() -> Result<Config, String> {
        let image_update_interval = match Config::get_update_interval() {
            Err(e) => return Err(e),
            Ok(interval) => interval,
        };
        let image_state_path = Config::get_image_state_path();
        let image_cache_path = Config::get_image_cache_path();

        Ok(Config {
            image_update_interval,
            image_cache_path,
            image_state_path,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct ImageState {
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    last_update: DateTime<Utc>,
    in_sync: bool,
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

fn fetch_state(image_state_path: String) -> Result<ImageState, Box<dyn std::error::Error>> {
    let state_cache = ImageState::open(image_state_path.clone());
    let state = match state_cache {
        Err(e) => {
            println!("WARNING: Could not read existing image state: {:?}", e);
            let state = ImageState {
                last_update: Utc::now(),
                in_sync: false,
            };
            let save_res = state.save(image_state_path);
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

fn copy_from_cache(image_cache_path: String) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::copy(image_cache_path, "./public/assets/daily_pic.jpg")?;
    Ok(())
}

fn update_image(
    image_cache_path: String,
    image_state_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = fetch_image(IMAGE_URL)?;

    // Update cache image
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(image_cache_path.clone())
        .unwrap();
    file.write(img.borrow()).unwrap();

    // Move cached image to to static
    copy_from_cache(image_cache_path)?;

    // Update state file
    let state = ImageState {
        last_update: Utc::now(),
        in_sync: true,
    };
    state.save(String::from(image_state_path))?;

    Ok(())
}

pub fn update_image_loop(config: Config) {
    // Copy cached image when initializing the loop
    let mut cache_ok = match copy_from_cache(config.image_cache_path.clone()) {
        Ok(_) => true,
        Err(e) => {
            println!("WARNING!, Failed to copy image from cache, {:?}", e);
            println!("Re-fetching image instead.");
            false
        }
    };

    loop {
        let image_update_interval = config.image_update_interval.clone();
        let image_cache_path = config.image_cache_path.clone();
        let image_state_path = config.image_state_path.clone();

        let state = fetch_state(image_state_path.clone());

        match state {
            Ok(state) => {
                // If we have a newly created image state, we need to fetch the image right away
                // Otherwise, wait for the update
                if state.in_sync & cache_ok {
                    wait_until_update(&state, image_update_interval);
                }

                println!("Updating image!");
                let update_res = update_image(image_cache_path, image_state_path);
                match update_res {
                    Ok(_) => {
                        cache_ok = true;
                    }
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

#[get("/daily_photo")]
fn daily_photo() -> Option<NamedFile> {
    NamedFile::open("./public/assets/daily_pic.jpg").ok()
}

pub fn start_web_server() {
    rocket::ignite().mount("/", routes![daily_photo]).launch();
}
