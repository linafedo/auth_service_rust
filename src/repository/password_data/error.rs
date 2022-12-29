#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid credentials.")]
    PasswordNotCorrect,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}