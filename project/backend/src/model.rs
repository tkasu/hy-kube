use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub task: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

#[derive(Debug)]
pub struct TodoListBoxed {
    pub todos: Arc<Mutex<TodoList>>,
}

impl TodoListBoxed {
    pub fn new() -> Self {
        let todo_vec: Vec<Todo> = Vec::new();
        let todo_list = TodoList { todos: todo_vec };

        Self {
            todos: Arc::new(Mutex::new(todo_list)),
        }
    }

    pub fn get_todos(&self) -> TodoList {
        let todos = (*self.todos.lock().unwrap()).clone();
        todos
    }

    pub fn add_todo(&self, todo: Todo) {
        let mut guard = self.todos.lock().unwrap();
        (*guard).todos.push(todo);
    }
}
