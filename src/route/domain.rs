use crate::utils::{MAX_LOGIN_LENGTH, MIN_PASSWORD_LENGTH, MIN_LOGIN_LENGTH, MAX_PASSWORD_LENGTH};
use crate::route::auth::login::RegistrationError;
use secrecy::{Secret, ExposeSecret};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct UserLogin(String);

impl UserLogin {
    pub fn parse(string: String) -> Result<UserLogin, RegistrationError> {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '-'];
        let contains_forbidden_characters = string
            .chars()
            .any(|c| forbidden_characters.contains(&c));

        if string.trim().is_empty() {
            return Err(RegistrationError::LoginIsEmpty)
        }
        if string.graphemes(true).count() > MAX_LOGIN_LENGTH
            || string.graphemes(true).count() < MIN_LOGIN_LENGTH {
            return Err(RegistrationError::LoginLengthIsWrong)
        }
        if contains_forbidden_characters {
            return Err(RegistrationError::LoginIsNotCorrect)
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
    pub fn parse(string: String) -> Result<UserPassword, RegistrationError> {
        if string.trim().is_empty()
            || string.graphemes(true).count() < MIN_PASSWORD_LENGTH
            || string.graphemes(true).count() > MAX_PASSWORD_LENGTH {
            return Err(RegistrationError::PasswordNotCorrect)
        }
        Ok(UserPassword { 0: Secret::new(string) })
    }

    pub fn expose_secret(&self) -> &String {
        self.0.expose_secret()
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}
