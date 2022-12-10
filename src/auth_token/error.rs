use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    WrongFileLength,
    DecodeSecretError(String),
    FileWithSecretNotFound(String),
    CreateFileForSecretError(String),
    WriteSecretToFileError(String),
    GenerateKeyError(String),
    SignTokenError(String),
    VerifyTokenError,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.to_string(), f)
    }
}