use diesel::prelude::*;
use crate::schema::tokens::dsl::*;
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

