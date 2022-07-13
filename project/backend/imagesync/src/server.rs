use rocket;
use rocket::fs::NamedFile;
use rocket::http::uri::Origin;
use rocket::http::Method;
use rocket::{Build, Rocket, Route};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
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

#[get("/")]
async fn healthcheck() -> &'static str {
    "Ok"
}

#[get("/daily_photo")]
async fn daily_photo(route: &Route, ip: IpAddr) -> Option<NamedFile> {
    ReqDetails::new(route, ip, None).log();
    NamedFile::open("./public/assets/daily_pic.jpg").await.ok()
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

    rocket::build()
        .mount("/", routes![healthcheck, daily_photo])
        .attach(cors)
}
