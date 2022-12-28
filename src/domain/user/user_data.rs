use crate::domain::user::error::DomainError;
use secrecy::{Secret, ExposeSecret};
use unicode_segmentation::UnicodeSegmentation;
use tracing::instrument;

pub const MIN_PASSWORD_LENGTH: usize = 6;
pub const MAX_PASSWORD_LENGTH: usize = 256;
pub const MAX_LOGIN_LENGTH: usize = 256;
pub const MIN_LOGIN_LENGTH: usize = 3;

#[derive(Debug, Clone)]
pub struct Login(pub Secret<String>);

impl Login {
    #[instrument(
        name = "Parsing user login from passed data",
        err
    )]
    pub fn parse(string: Secret<String>) -> Result<Login, DomainError> {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '-'];
        let contains_forbidden_characters = string.expose_secret()
            .chars()
            .any(|c| forbidden_characters.contains(&c));

        if string.expose_secret().trim().is_empty() {
            return Err(DomainError::LoginIsEmpty)
        }
        if string.expose_secret().graphemes(true).count() > MAX_LOGIN_LENGTH
            || string.expose_secret().graphemes(true).count() < MIN_LOGIN_LENGTH {
            return Err(DomainError::LoginLengthIsWrong)
        }
        if contains_forbidden_characters {
            return Err(DomainError::LoginIsNotCorrect)
        }
        Ok(Login { 0: string })
    }
}

impl AsRef<str> for Login {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

#[derive(Debug, Clone)]
pub struct Password(pub Secret<String>);

impl Password {
    #[instrument(
        name = "Parsing password_data from passed data",
        err
    )]
    pub fn parse(string: Secret<String>) -> Result<Password, DomainError> {
        if string.expose_secret().trim().is_empty()
            || string.expose_secret().graphemes(true).count() < MIN_PASSWORD_LENGTH
            || string.expose_secret().graphemes(true).count() > MAX_PASSWORD_LENGTH {
            return Err(DomainError::PasswordNotCorrect)
        }
        Ok(Password { 0: Secret::new(string.expose_secret().clone()) })
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

#[derive(Debug, Clone)]
pub struct PasswordData {
    pub password_hash: Secret<String>,
    pub salt: Secret<String>,
}