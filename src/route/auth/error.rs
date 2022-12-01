use actix_web::ResponseError;
use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Password is not correct")]
    PasswordNotCorrect,
    #[error("User not exist")]
    UserNotExist,
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticationError::PasswordNotCorrect => StatusCode::BAD_REQUEST,
            AuthenticationError::UserNotExist => StatusCode::BAD_REQUEST,
        }
    }
}