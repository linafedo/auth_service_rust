pub mod auth;
use rocket_contrib::database;
use rocket_contrib::databases::diesel as rocket_diesel;
use diesel::PgConnection as diesel_connection;

#[database("users")]
pub struct DatabaseConnection(diesel_connection);
