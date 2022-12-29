use actix_web::ResponseError;
use actix_web::http::StatusCode;
use crate::route::dto;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Password is not correct")]
    PasswordNotCorrect,
    #[error("User not exist")]
    UserNotExist,
    #[error("Something went wrong")]
    UnexpectedError,
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticationError::PasswordNotCorrect => StatusCode::CONFLICT,
            AuthenticationError::UserNotExist => StatusCode::CONFLICT,
            AuthenticationError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl AuthenticationError {
    pub fn password_not_correct_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(
            AuthenticationError::PasswordNotCorrect
        )
    }

    pub fn user_not_exist_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(
            AuthenticationError::UserNotExist
        )
    }

    pub fn unexpected_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(
            AuthenticationError::UnexpectedError
        )
    }
}