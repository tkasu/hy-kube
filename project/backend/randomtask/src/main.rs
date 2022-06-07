use randomtask::todo;
use randomtask::url_fetcher;
use randomtask::{api, config};

fn main() {
    let url = url_fetcher::get_random_url();
    let todo = todo::todo_from_url(url);

    let api_endpoint = config::get_api_endpoint_url();
    api::send_todo_to_api(&api_endpoint, &todo);

    println!(
        "Sent random todo: {:?} to api url: {:?}",
        todo, api_endpoint
    );
}
