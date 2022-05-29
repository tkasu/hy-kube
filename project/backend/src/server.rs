use crate::db;
use crate::db::ProjectDbConn;
use crate::model::{Todo, TodoList};
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use rocket_db_pools::Database;

#[get("/daily_photo")]
async fn daily_photo() -> Option<NamedFile> {
    NamedFile::open("./public/assets/daily_pic.jpg").await.ok()
}

#[get("/todos")]
async fn todos(db_conn: &ProjectDbConn) -> Json<TodoList> {
    let todos = db::get_todos(db_conn).await;
    Json(todos)
}

#[post("/todo", format = "json", data = "<todo>")]
async fn new_todo(db_conn: &ProjectDbConn, todo: Json<Todo>) -> Json<Todo> {
    db::add_todo(db_conn, todo.clone().0).await;
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
    let cors = get_cors();
    let db_conn = ProjectDbConn::init();

    rocket::build()
        .mount("/", routes![daily_photo, todos, new_todo])
        .attach(cors)
        .attach(db_conn)
        .attach(AdHoc::try_on_ignite("DB Migrations", db::run_migrations))
}
