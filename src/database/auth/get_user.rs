use crate::database::auth::models::{GetUser, User};
use crate::schema::users::dsl::*;
use crate::schema::tokens::dsl::*;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;


pub fn with_token(db: &mut diesel::PgConnection, with_token: &str) -> GetUser {
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

pub fn with_credentials(db: &mut diesel::PgConnection, user_login: &str, with_password: &str) -> GetUser {
     match users
         .filter(login.eq(user_login.to_lowercase()))
         .get_result::<User>(db) {
          Ok(user) => {
               let argon_default = Argon2::default();
               if let Ok(parsed_hash) = PasswordHash::new(&user.password) {
                    // todo: - add hash for password
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