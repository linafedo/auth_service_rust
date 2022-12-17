use crate::domain::user::error::DomainError;
use crate::utils;

use secrecy::{Secret, ExposeSecret};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct UserLogin(String);

impl UserLogin {
    pub fn parse(string: String) -> Result<UserLogin, DomainError> {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '-'];
        let contains_forbidden_characters = string
            .chars()
            .any(|c| forbidden_characters.contains(&c));

        if string.trim().is_empty() {
            tracing::error!("{:?}", DomainError::LoginIsEmpty);
            return Err(DomainError::LoginIsEmpty)
        }
        if string.graphemes(true).count() > utils::MAX_LOGIN_LENGTH
            || string.graphemes(true).count() < utils::MIN_LOGIN_LENGTH {
            tracing::error!("{:?}", DomainError::LoginLengthIsWrong);
            return Err(DomainError::LoginLengthIsWrong)
        }
        if contains_forbidden_characters {
            tracing::error!("{:?}", DomainError::LoginIsNotCorrect);
            return Err(DomainError::LoginIsNotCorrect)
        }
        Ok(UserLogin{ 0: string })
    }
}

impl AsRef<str> for UserLogin {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct UserPassword(Secret<String>);

impl UserPassword {
    pub fn parse(string: String) -> Result<UserPassword, DomainError> {
        if string.trim().is_empty()
            || string.graphemes(true).count() < utils::MIN_PASSWORD_LENGTH
            || string.graphemes(true).count() > utils::MAX_PASSWORD_LENGTH {
            tracing::error!("{:?}", DomainError::PasswordNotCorrect);
            return Err(DomainError::PasswordNotCorrect)
        }
        Ok(UserPassword { 0: Secret::new(string) })
    }

    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl AsRef<str> for UserPassword {
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