use crate::model::{Todo, TodoList, TodoListBoxed};
use rocket::fs::NamedFile;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

#[get("/daily_photo")]
async fn daily_photo() -> Option<NamedFile> {
    NamedFile::open("./public/assets/daily_pic.jpg").await.ok()
}

#[get("/todos")]
fn todos(todo_list: &State<TodoListBoxed>) -> Json<TodoList> {
    let todos = todo_list.get_todos();
    Json(todos)
}

#[post("/todo", format = "json", data = "<todo>")]
fn new_todo(todo_list: &State<TodoListBoxed>, todo: Json<Todo>) -> Json<Todo> {
    let todo_add = todo.clone();
    todo_list.add_todo(todo_add.0);
    todo
}

fn get_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error creating CORS fairing");

    cors
}

pub fn build_web_server() -> Rocket<Build> {
    let todo_state = TodoListBoxed::new();
    let cors = get_cors();

    rocket::build()
        .manage(todo_state)
        .mount("/", routes![daily_photo, todos, new_todo])
        .attach(cors)
}
