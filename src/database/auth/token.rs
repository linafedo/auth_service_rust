use uuid::Uuid;
use diesel::prelude::*;

pub enum CreateToken {
    Ok(String),
    Error
}

#[derive(Queryable)]
pub struct NewToken<'a>  {
    user_id: &'a Uuid,
    token: &'a str
}
