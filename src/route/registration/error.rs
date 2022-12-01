use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("Password must contain at least 6 characters")]
    PasswordNotCorrect,
    #[error("User already exists")]
    AlreadyExist,
    #[error("Login must contain from 3 to 256 characters")]
    LoginLengthIsWrong,
    #[error("Login should be contain only letters and numbers and start with a letter")]
    LoginIsNotCorrect,
    #[error("Login should be not empty")]
    LoginIsEmpty,
    #[error("Password hashing error")]
    PasswordHashError
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::PasswordNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::AlreadyExist => StatusCode::INTERNAL_SERVER_ERROR,
            RegistrationError::LoginLengthIsWrong => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsEmpty => StatusCode::BAD_REQUEST,
            RegistrationError::PasswordHashError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}