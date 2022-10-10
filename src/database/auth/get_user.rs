use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::schema::users::dsl::*;
use crate::database::auth::models::{GetUser, User};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;
use diesel::expression::AsExpression;
use rocket_contrib::databases::diesel::serialize::ToSql;

pub fn with_credentials(db: &mut diesel::PgConnection, login: &str, with_password: &str) -> GetUser {
     match users
         .filter(user_login.eq(login.to_lowercase()))
         .get_result::<User>(db) {
          Ok(user) => {
               let argon_default = Argon2::default();
               if let Ok(parsed_hash) = PasswordHash::new(&user.password) {
                    // зачем мы тут хэшируем пассворд, если это можно делать при создании учетки?
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