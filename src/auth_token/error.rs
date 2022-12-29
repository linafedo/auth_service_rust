use std::fmt::Debug;
use anyhow::Context;

#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Verify token Error")]
    VerifyTokenError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}