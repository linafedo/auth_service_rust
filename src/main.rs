#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod config;
pub mod database;
mod schema;
pub mod handlers;
mod routes;

fn main() {
    rocket::custom(config::from_env())
        .launch();
    println!("Hello, world!");
}
