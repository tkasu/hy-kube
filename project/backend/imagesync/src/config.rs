use std::env;

pub const IMAGE_URL: &'static str = "https://picsum.photos/1200";
pub const DEFAULT_IMAGE_CACHE_PATH: &'static str = "index_image.jpg";
pub const DEFAULT_IMAGE_STATE_PATH: &'static str = "index_state.json";
pub const DEFAULT_IMAGE_UPDATE_INTERVAL_SECS: i64 = 60;

#[derive(Clone, Debug)]
pub struct Config {
    pub image_update_interval: i64,
    pub image_cache_path: String,
    pub image_state_path: String,
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
