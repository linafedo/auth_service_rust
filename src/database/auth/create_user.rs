use crate::database::auth::get_user::User;
use crate::schema::users::dsl::*;
use crate::schema::users;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;
use diesel::{Insertable, RunQueryDsl, result};
use diesel;

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub password: &'a str,
    pub login: &'a str
}

pub enum CreateUser {
    Ok,
    WeakPassword,
    AlreadyExist,
    Other
}

const PASSWORD_LENGHT: usize = 8;

pub fn create_user(db: &diesel::PgConnection, user_login: &str, user_password: &str) -> CreateUser {
    if user_password.len() < PASSWORD_LENGHT { return CreateUser::WeakPassword }

    let salt = SaltString::generate(OsRng);
    let argon_default = Argon2::default();
    let password_hash = argon_default
        .hash_password_simple(user_password.as_bytes(), &salt)
        .unwrap();
    let new_user = NewUser { password: &password_hash.to_string(), login: user_login };
    match diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(db) {
        Ok(_) => CreateUser::Ok,
        Err(result::Error::DatabaseError(
                result::DatabaseErrorKind::UniqueViolation, _,
            )) => CreateUser::AlreadyExist,
        Err(_) => CreateUser::Other,
    }
}