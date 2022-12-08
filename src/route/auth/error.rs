use actix_web::ResponseError;
use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Password is not correct")]
    PasswordNotCorrect,
    #[error("User not exist")]
    UserNotExist,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticationError::PasswordNotCorrect => StatusCode::CONFLICT,
            AuthenticationError::UserNotExist => StatusCode::CONFLICT,
            AuthenticationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}