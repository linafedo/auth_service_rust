use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Verify token Error")]
    VerifyTokenError,
    #[error("File with secret not found")]
    FileWithSecretNotFound,
    #[error("Something went wrong")]
    UnexpectedError,
}