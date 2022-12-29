use crate::repository::password_data::error::Error;
use crate::domain::user::user_data::PasswordData;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use secrecy::{ExposeSecret, Secret};
use tracing::instrument;
use anyhow::Context;

#[instrument(
    name = "Generate password_data hash for new user",
    err
)]
pub fn generate(password: Secret<String>) -> Result<PasswordData, Error> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash_password = argon.hash_password(
        password.expose_secret().as_bytes(),
        salt.as_ref(),
    )
        .map_err(|_|
            Error::UnexpectedError(anyhow::Error::msg(
                "Output for hash password is empty during generate new password")
            ))?;

    if let Some(output) = hash_password.hash {
        Ok(PasswordData {
            password_hash: Secret::new(base64::encode(output.as_bytes())),
            salt: Secret::new(salt.as_str().to_string()),
        })
    } else {
        return Err(
            Error::UnexpectedError(anyhow::Error::msg(
                    "Output for hash password is empty during generate new password")
            )
        )
    }
}

#[instrument(
    name = "Checking passed password_data with user password_data in database",
    err
)]
pub fn check_password(
    password: Secret<String>,
    salt: Secret<String>,
    password_hash: Secret<String>) -> Result<(), Error> {
    let argon = Argon2::default();
    let hash_password = argon.hash_password(
        password.expose_secret().as_bytes(),
        salt.expose_secret()
    )
        .map_err(|_|
            Error::UnexpectedError(anyhow::Error::msg(
                "Generate hash password for check passed password failed")
            ))?;
    return if let Some(output) = hash_password.hash {
        if base64::encode(output.as_bytes()) == password_hash.expose_secret().clone() {
            Ok(())
        } else {
            Err(Error::PasswordNotCorrect)
        }
    } else {
        Err(
            Error::UnexpectedError(
                anyhow::Error::msg(
                    "Output for hash password is empty during check passed password"
                )
            )
        )
    }
}