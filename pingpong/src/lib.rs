#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

pub mod config;
pub mod db_init;
pub mod db_live;
pub mod models;
pub mod schema;
pub mod server;
