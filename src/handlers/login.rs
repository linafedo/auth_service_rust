use crate::database::auth::get_user;
use crate::database;
use crate::database::auth::get_user::GetUser;
use crate::database::auth::token::CreateToken;
use crate::database::DatabaseConnection;

pub enum LoginError {
    NotFound,
    Other
}

pub enum LoginResult {
    Success(String),
    Failed(LoginError)
}

pub fn login(login: &str, password: &str, db: DatabaseConnection) -> LoginResult {
    match get_user::with_credentials(&*db, login, password) {
        GetUser::Some(user) => {
            match database::auth::token::create_token(&*db, &user) {
                CreateToken::Ok(token) => LoginResult::Success(token),
                CreateToken::Error => LoginResult::Failed(LoginError::Other),
            }
        }
        GetUser::None => LoginResult::Failed(LoginError::NotFound),
        GetUser::Error => LoginResult::Failed(LoginError::Other),
    }
}