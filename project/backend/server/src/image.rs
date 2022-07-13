use crate::config;
use bytes::Bytes;
use chrono::Duration;
use chrono::Utc;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread::sleep;

fn wait_until_update(image_update_interval: i64) {
    let current_time = Utc::now();
    let next_update = current_time
        .checked_add_signed(Duration::seconds(image_update_interval))
        .unwrap();
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

fn fetch_image(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.error_for_status()?;
    let image = resp.bytes().unwrap();
    Ok(image)
}

fn update_image(config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let img = fetch_image(config.image_sync_endpoint_url.as_str())?;

    // Update cache image
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("./public/assets/daily_pic.jpg")
        .unwrap();
    file.write(img.borrow()).unwrap();

    Ok(())
}

pub fn update_image_loop(config: config::Config) {
    let image_sync_interval = config.image_sync_interval.clone();

    loop {
        match update_image(&config) {
            Ok(_) => {
                println!("Image updated");
                wait_until_update(image_sync_interval);
            }
            Err(e) => {
                println!("WARNING!, Failed to sync image, {:?}", e);
                println!("Retrying in 10 seconds.");
                sleep(std::time::Duration::from_secs(10));
            }
        };
    }
}
