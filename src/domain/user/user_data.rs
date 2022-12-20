use crate::domain::user::error::DomainError;
use crate::utils;

use secrecy::{Secret, ExposeSecret};
use unicode_segmentation::UnicodeSegmentation;
use tracing::instrument;

#[derive(Debug)]
pub struct Login(String);

impl Login {
    #[instrument(
        name = "Parsing user login from passed data",
        err
    )]
    pub fn parse(string: String) -> Result<Login, DomainError> {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '-'];
        let contains_forbidden_characters = string
            .chars()
            .any(|c| forbidden_characters.contains(&c));

        if string.trim().is_empty() {
            return Err(DomainError::LoginIsEmpty)
        }
        if string.graphemes(true).count() > utils::MAX_LOGIN_LENGTH
            || string.graphemes(true).count() < utils::MIN_LOGIN_LENGTH {
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
        &self.0
    }
}

#[derive(Debug)]
pub struct Password(Secret<String>);

impl Password {
    #[instrument(
        name = "Parsing password from passed data",
        skip(string),
        err
    )]
    pub fn parse(string: String) -> Result<Password, DomainError> {
        if string.trim().is_empty()
            || string.graphemes(true).count() < utils::MIN_PASSWORD_LENGTH
            || string.graphemes(true).count() > utils::MAX_PASSWORD_LENGTH {
            return Err(DomainError::PasswordNotCorrect)
        }
        Ok(Password { 0: Secret::new(string) })
    }

    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

#[derive(Debug, Clone)]
pub struct PasswordData {
    pub password_hash: String,
    pub salt: Secret<String>,
}

impl PasswordData {
    pub fn expose_salt_secret(&self) -> &str {
        self.salt.expose_secret()
    }
}