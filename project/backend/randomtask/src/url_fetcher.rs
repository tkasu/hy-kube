use crate::config;

pub fn get_random_url() -> String {
    let resp = reqwest::blocking::get(config::RANDOM_ARTICLE_URL).unwrap();
    resp.url().to_string()
}
