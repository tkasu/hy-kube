use std::env;

pub const RANDOM_ARTICLE_URL: &'static str = "https://en.wikipedia.org/wiki/Special:Random";
pub const ADD_TODO_ENDPOINT: &'static str = "/todo";

fn get_api_url() -> String {
    env::var("API_URL").expect("API_URL must be set")
}

pub fn get_api_endpoint_url() -> String {
    let api_url = get_api_url();
    format!("{}{}", api_url, ADD_TODO_ENDPOINT)
}
