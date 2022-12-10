use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::domain::user::error::DomainError;

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
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::PasswordNotCorrect => StatusCode::CONFLICT,
            RegistrationError::AlreadyExist => StatusCode::CONFLICT,
            RegistrationError::LoginLengthIsWrong => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsEmpty => StatusCode::BAD_REQUEST,
            RegistrationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            DomainError::HashingError=> {
                RegistrationError::UnexpectedError(anyhow::Error::from(domain_error))
            }
        }
    }
}