use crate::schema::users::dsl::*;
use crate::schema::tokens::dsl::*;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, Queryable};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable ,PartialEq, Debug)]
pub struct User {
     pub password: String,
     pub login: String,
     pub id: Uuid,
}

pub enum GetUser {
     Some(User),
     None,
     Error
}

pub fn with_token(db: &diesel::PgConnection, with_token: &str) -> GetUser {
     match users
         .left_join(tokens.on(user_id.eq(id)))
         .select((password, login, id))
         .filter(token.eq(with_token))
         .get_result::<User>(db)
     {
          Ok(user) => GetUser::Some(user),
          Err(diesel::result::Error::NotFound) => GetUser::None,
          _ => GetUser::Error,
     }
}

pub fn with_credentials(db: &diesel::PgConnection, user_login: &str, with_password: &str) -> GetUser {
     match users
         .filter(login.eq(user_login.to_lowercase()))
         .get_result::<User>(db) {
          Ok(user) => {
               let argon_default = Argon2::default();
               if let Ok(parsed_hash) = PasswordHash::new(&user.password) {
                    if argon_default.verify_password(with_password.as_bytes(), &parsed_hash)
                        .is_ok() {
                         GetUser::Some(user)
                    } else {
                         GetUser::None
                    }
               } else {
                    GetUser::None
               }
          }
          Err(diesel::result::Error::NotFound) => GetUser::None,
          _ => GetUser::Error,
     }
}