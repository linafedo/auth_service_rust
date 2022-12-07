use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenManagerError {
    #[error("[Error generating token]")]
    GenerateTokenError,
    #[error("Error signing with key")]
    SignWithKeyError,
    #[error("Error reading token secret")]
    ReadSecretKeyError
}