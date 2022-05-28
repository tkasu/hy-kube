use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Todo {
    pub task: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}
