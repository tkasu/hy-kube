use std::env;

pub const IMAGE_URL: &'static str = "https://picsum.photos/1200";
pub const DEFAULT_IMAGE_SYNC_INTERVAL_SECS: i64 = 60;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub image_sync_interval: i64,
    pub image_sync_endpoint_url: String,
}

impl Config {
    fn get_db_url() -> String {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }

    fn get_image_sync_endpoint_url() -> String {
        env::var("IMAGE_SYNC_URL").expect("IMAGE_SYNC_URL must be set")
    }

    fn get_sync_interval() -> Result<i64, String> {
        let image_sync_interval: Result<i64, String> = match env::var("IMAGE_SYNC_INTERVAL") {
            Err(_) => {
                let sync_interval = DEFAULT_IMAGE_SYNC_INTERVAL_SECS;
                Ok(sync_interval)
            }
            Ok(sync_interval) => {
                let sync_interval = match sync_interval.parse::<i64>() {
                    Ok(interval) => Ok(interval),
                    Err(e) => {
                        let err_str = format!(
                            "Failed to parse IMAGE_SYNC_INTERVAL `{}`: {}",
                            sync_interval, e
                        );
                        Err(err_str)
                    }
                };
                sync_interval
            }
        };

        image_sync_interval
    }

    pub fn new() -> Result<Config, String> {
        let db_url = Config::get_db_url();
        let image_sync_endpoint_url = Config::get_image_sync_endpoint_url();
        let image_sync_interval = match Config::get_sync_interval() {
            Err(e) => return Err(e),
            Ok(interval) => interval,
        };

        Ok(Config {
            db_url,
            image_sync_endpoint_url,
            image_sync_interval,
        })
    }
}
