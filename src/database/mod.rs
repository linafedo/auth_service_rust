pub mod auth;
use rocket_contrib::database;
use diesel::PgConnection as diesel_connection;

#[database("users")]
pub struct DatabaseConnection(diesel_connection);
