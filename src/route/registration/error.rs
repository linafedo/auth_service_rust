use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
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

impl RegistrationError {
    pub fn get_internal_error_code(&self) -> &str {
        match self {
            RegistrationError::PasswordNotCorrect => "registration_001",
            RegistrationError::AlreadyExist => "registration_002",
            RegistrationError::LoginLengthIsWrong => "registration_003",
            RegistrationError::LoginIsNotCorrect => "registration_004",
            RegistrationError::LoginIsEmpty => "registration_005",
            RegistrationError::UnexpectedError(_) => "registration_006",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            RegistrationError::PasswordNotCorrect => "PasswordNotCorrect",
            RegistrationError::AlreadyExist => "AlreadyExist",
            RegistrationError::LoginLengthIsWrong => "LoginLengthIsWrong",
            RegistrationError::LoginIsNotCorrect => "LoginIsNotCorrect",
            RegistrationError::LoginIsEmpty => "LoginIsEmpty",
            RegistrationError::UnexpectedError(_) => "UnexpectedError",
        }
    }
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

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: self.get_internal_error_code().to_string(),
            message: self.to_string(),
            error: self.name().to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
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

#[derive(serde::Serialize)]
struct ErrorResponse {
    code: String,
    error: String,
    message: String,
}