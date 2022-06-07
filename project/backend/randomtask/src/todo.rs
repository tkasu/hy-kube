use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub task: String,
}

pub fn todo_from_url(url: String) -> Todo {
    let task = format!("Read: {}", url);
    Todo { task }
}
