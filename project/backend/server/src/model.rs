use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Todo {
    pub task: String,
}

impl Todo {
    pub fn len(&self) -> usize {
        self.task.len()
    }

    pub fn to_untyped_json(&self) -> Value {
        let str_params = serde_json::to_string(&self).unwrap();
        let json_params: Value = serde_json::from_str(str_params.as_str()).unwrap();
        json_params
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}
