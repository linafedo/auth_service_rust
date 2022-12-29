use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::repository::password_data::error::Error;
use crate::route::dto;

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("User already exists")]
    AlreadyExist,
    #[error(transparent)]
    DataIsNotCorrect(#[from] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError,
}

impl RegistrationError {
    pub fn password_not_correct_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(
            RegistrationError::DataIsNotCorrect(
                anyhow::Error::msg("Password is not correct.")
            )
        )
    }

    pub fn user_exist_error_example() -> dto::error::ResponseError {
        dto::error::ResponseError::from_error(
            RegistrationError::DataIsNotCorrect(
                anyhow::Error::msg("User already exists")
            )
        )
    }
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::AlreadyExist => StatusCode::CONFLICT,
            RegistrationError::DataIsNotCorrect(_) => StatusCode::BAD_REQUEST,
            RegistrationError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}