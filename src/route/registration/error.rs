use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use sqlx::Error;
use crate::domain::user::error::DomainError;
use crate::route::dto;

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
    #[error("Something went wrong")]
    UnexpectedError,
}

impl RegistrationError {
    pub fn password_not_correct_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(RegistrationError::PasswordNotCorrect)
    }

    pub fn user_exist_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(RegistrationError::AlreadyExist)
    }
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::PasswordNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::AlreadyExist => StatusCode::CONFLICT,
            RegistrationError::LoginLengthIsWrong => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsEmpty => StatusCode::BAD_REQUEST,
            RegistrationError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DomainError> for RegistrationError {
    fn from(domain_error: DomainError) -> Self {
        match domain_error {
            DomainError::LoginIsEmpty=> {
                RegistrationError::LoginIsEmpty
            }
            DomainError::LoginLengthIsWrong=> {
                RegistrationError::LoginLengthIsWrong
            }
            DomainError::LoginIsNotCorrect=> {
                RegistrationError::LoginIsNotCorrect
            }
            DomainError::PasswordNotCorrect=> {
                RegistrationError::PasswordNotCorrect
            }
            DomainError::UnexpectedError=> {
                RegistrationError::UnexpectedError
            }
        }
    }
}