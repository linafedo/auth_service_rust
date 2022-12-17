use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use secrecy::Secret;
use crate::domain::user::error::DomainError;
use crate::domain::user::user_data::PasswordData;

pub fn generate(password: &str) -> Result<PasswordData, DomainError> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash_password = argon.hash_password(
        password.as_bytes(),
        salt.as_ref()
    ).map_err(|e| {
        tracing::error!("Password hash generation error - {:?}", e.to_string());
        DomainError::UnexpectedError
    })?;

    if let Some(output) = hash_password.hash {
        Ok(PasswordData {
            password_hash: base64::encode(output.as_bytes()),
            salt: Secret::new(salt.as_str().to_string()),
        })
    } else {
        tracing::error!("Password hash is null");
        return Err(DomainError::UnexpectedError)
    }
}

pub fn check_password(password: &str, salt: &str, password_hash: &str) -> Result<(), DomainError> {
    let argon = Argon2::default();
    let hash_password = argon.hash_password(
        password.as_bytes(),
        salt
    ).map_err(|e| {
        tracing::error!("Password hash generation for check returned error - {:?}",e.to_string());
        DomainError::UnexpectedError
    })?;

    return if let Some(output) = hash_password.hash {
        if base64::encode(output.as_bytes()) == password_hash {
            Ok(())
        } else {
            tracing::error!("{:?}", DomainError::PasswordNotCorrect);
            Err(DomainError::PasswordNotCorrect)
        }
    } else {
        tracing::error!("Password hash for check is null");
        Err(DomainError::UnexpectedError)
    }
}