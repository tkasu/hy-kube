use crate::todo::Todo;

pub fn send_todo_to_api(api_endpoint_url: &String, todo: &Todo) {
    let client = reqwest::blocking::Client::new();

    client
        .post(api_endpoint_url)
        .json(&todo)
        .send()
        .unwrap()
        .error_for_status()
        .unwrap();
}
