use crate::db;
use crate::db::ProjectDbConn;
use crate::model::{Todo, TodoList};
use rocket;
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::http::uri::Origin;
use rocket::http::Method;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, Route};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ReqDetails<'a> {
    details_type: &'static str,
    method: Method,
    uri: Origin<'a>,
    source_ip: IpAddr,
    params: Option<Value>,
}

impl ReqDetails<'_> {
    fn new(route: &Route, ip: IpAddr, params: Option<Value>) -> Self {
        Self {
            details_type: "request",
            method: route.method,
            uri: route.uri.origin.clone(),
            source_ip: ip,
            params,
        }
    }

    fn log(&self) {
        let json = serde_json::json!(self).to_string();
        println!("{:?}", json)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum RespErr {
    BadRequest(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ErrDetails {
    detail_type: &'static str,
    err: RespErr,
}

impl ErrDetails {
    fn from_bad_req(err: &status::BadRequest<String>) -> Self {
        let resp_err = RespErr::BadRequest(err.clone().0.unwrap());
        Self {
            detail_type: "error",
            err: resp_err,
        }
    }

    fn log(&self) {
        let json = serde_json::json!(self).to_string();
        println!("{:?}", json)
    }
}

#[get("/")]
async fn healthcheck() -> &'static str {
    "Ok"
}

#[get("/api/daily_photo")]
async fn daily_photo(route: &Route, ip: IpAddr) -> Option<NamedFile> {
    ReqDetails::new(route, ip, None).log();
    NamedFile::open("./public/assets/daily_pic.jpg").await.ok()
}

#[get("/api/todos")]
async fn todos<'a>(db_conn: &ProjectDbConn, route: &Route, ip: IpAddr) -> Json<TodoList> {
    ReqDetails::new(route, ip, None).log();

    let todos = db::get_todos(db_conn).await;
    Json(todos)
}

#[post("/api/todo", format = "json", data = "<todo>")]
async fn new_todo<'a>(
    todo: Json<Todo>,
    db_conn: &ProjectDbConn,
    route: &Route,
    ip: IpAddr,
) -> Result<Json<Todo>, status::BadRequest<String>> {
    let json_params = todo.to_untyped_json();
    ReqDetails::new(route, ip, Some(json_params)).log();

    let todo_len = todo.len();
    if todo_len > 140 {
        let error_msg = format!("Todo's length: {} is over the limit 140.", todo_len);
        let err_status = status::BadRequest(Some(error_msg));
        //println!("Err Resp: {:?}", err_status);
        ErrDetails::from_bad_req(&err_status).log();
        return Err(err_status);
    }
    db::add_todo(db_conn, todo.clone().0).await;
    Ok(todo)
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
        .mount("/", routes![healthcheck, daily_photo, todos, new_todo])
        .attach(cors)
        .attach(db_conn)
        .attach(AdHoc::try_on_ignite("DB Migrations", db::run_migrations))
}
