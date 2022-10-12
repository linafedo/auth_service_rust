use diesel::prelude::*;
use diesel::query_dsl::LoadQuery;
use diesel::sql_types;
use uuid::Uuid;

// Models

#[derive(Queryable ,PartialEq, Debug)]
pub struct User {
    pub password: String,
    pub login: String,
    pub id: Uuid,
}

#[derive(Queryable)]
pub struct NewToken<'a>  {
    id: &'a Uuid,
    token: &'a str
}

// Status states

pub enum GetUser {
    Some(User),
    None,
    Error
}

pub enum CreateToken {
    Ok(String),
    Error
}

