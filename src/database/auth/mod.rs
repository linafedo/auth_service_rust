use diesel::prelude::*;
use uuid::Uuid;

mod get_user;
mod token;

#[derive(Queryable)]
pub struct User {
    user_id: Uuid,
    user_login: String,
    password: String
}



