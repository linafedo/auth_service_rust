use crate::utils::{MAX_LOGIN_LENGTH, MIN_PASSWORD_LENGTH, MIN_LOGIN_LENGTH, MAX_PASSWORD_LENGTH};
use crate::route::registration::error::RegistrationError;

use std::io::Read;
use secrecy::{Secret, ExposeSecret};
use unicode_segmentation::UnicodeSegmentation;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::Encoding::B64;
use rand_core::OsRng;
use sqlx::Encode;

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

pub struct PasswordData {
    password_hash: String,
    salt: String,
}

impl PasswordData {
    pub fn get_password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn get_salt(&self) -> &str {
        &self.salt
    }

    pub fn generate(password: &String) -> Result<Self, RegistrationError> {
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        match argon.hash_password_simple(
            password.as_bytes(),
            salt.as_ref()
        ) {
            Ok(result) => {
                let output = match result.hash {
                    Some(result) => result,
                    None => {
                        tracing::error!("Password hash is null");
                        return Err(RegistrationError::PasswordHashError)
                    }
                };
                Ok( PasswordData {
                    password_hash: base64::encode(output.as_bytes()),
                    salt: salt.as_str().to_string(),
                })
            },
            Err(e) => {
                tracing::error!("Password hash generation error - {:?}", e.to_string());
                return Err(RegistrationError::PasswordHashError)
            }
        }
    }

    pub fn check_password(password: &str, salt: &str, password_hash: &str) -> Result<(), ()> {
        let argon = Argon2::default();

        return match argon.hash_password_simple(
            password.as_bytes(),
            salt
        ) {
            Ok(result) => {
                let output = match result.hash {
                    Some(result) => result,
                    None => {
                        tracing::error!("Password hash for check is null");
                        return Err(())
                    }
                };
                if base64::encode(output.as_bytes()) == password_hash { return Ok(()) };
                Err(())
            },
            Err(e) => {
                tracing::error!(
                    "Password hash generation for check returned error - {:?}",
                    e.to_string()
                );
                Err(())
            }
        }
    }
}