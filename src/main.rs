#![feature(decl_macro)]
#![feature(let_else)]
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
mod env_var;

fn main() {
    match config::from_env() {
        Ok(config) => {
            rocket::custom(config)
                .launch();
        }
        Err(e) => {
            println!("Error in env config: - {e}");
            todo!()
        }
    }
}
