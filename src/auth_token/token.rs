use crate::auth_token::error::TokenError;

use jwt::{SignWithKey, VerifyWithKey, Header, Token};
use chrono::{Duration};
use hmac::digest::KeyInit;
use hmac::Hmac;
use serde::{Serialize, Deserialize};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use std::io::Write;
use std::fs;
use uuid::Uuid;
use tracing::instrument;
use secrecy::{ExposeSecret, Secret};

#[derive(Default, Deserialize, Serialize)]
struct TokenData {
    user_id: String,
    expiration_timestamp: i64,
}

pub struct SecretKey {
    value: Secret<[u8; 64]>
}

impl SecretKey {
    fn new(value: Secret<[u8; 64]>) -> Self {
        SecretKey{ value }
    }
}

#[instrument(
    name = "Generating of new token for user",
    err
)]
pub fn new_token(user_id: Uuid, duration_in_days: i64) -> Result<String, TokenError> {
    // header
    let header: Header = Default::default();
    // claims
    let expiration_time = chrono::Utc::now() + Duration::days(duration_in_days);

    let claims = TokenData {
        user_id: user_id.to_string(),
        expiration_timestamp: expiration_time.timestamp(),
    };
    // key
    let secret_key = read_or_generate_secret_key()?;

    let unsigned_token = Token::new(header, claims);
    let signed_token = unsigned_token.sign_with_key(&secret_key).map_err(|_|
        TokenError::UnexpectedError
    )?;
    Ok(signed_token.into())
}


#[instrument(
    name = "Verifying of token",
    err
)]
pub fn verify_token(token: Secret<String>) -> Result<(), TokenError> {
    let secret = read_secret_from_file()
        .map_err(|_|
        TokenError::UnexpectedError
    )?;
    let generated_key: Hmac<Sha256> = Hmac::new_from_slice(secret.value.expose_secret())
        .map_err(|_|
            TokenError::UnexpectedError
        )?;

    VerifyWithKey::verify_with_key(token.expose_secret().as_str(), &generated_key).map_err(|_|
        TokenError::VerifyTokenError
    )?;
    Ok(())

}

fn read_or_generate_secret_key() -> Result<Hmac<Sha256>, TokenError> {
    if let Ok(secret) = read_secret_from_file() {
        let key = Hmac::new_from_slice(secret.value.expose_secret())
            .map_err(|_|
            TokenError::UnexpectedError
        )?;
        Ok(key)
    } else {
        Ok(create_and_save_secret_key()?)
    }
}

#[instrument(
    name = "Creating and saving secret",
    err
)]
fn create_and_save_secret_key() -> Result<Hmac<Sha256>, TokenError> {
    let mut key: [u8; 64] = [0; 64];
    let mut default = OsRng::default();
    default.fill_bytes(&mut key);
    let secret_key = SecretKey::new(Secret::new(key));

    let generated_key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.value.expose_secret())
        .map_err(|_| {
        TokenError::UnexpectedError
    })?;
    save_secret(secret_key).map_err(|_| TokenError::UnexpectedError)?;
    Ok(generated_key)
}

#[instrument(
    name = "Save secret to file",
    err
)]
fn save_secret(secret: SecretKey) -> Result<(), TokenError> {
    let mut file = fs::File::create_new("token_secret.txt").map_err(|_|
        TokenError::UnexpectedError
    )?;

    let secret = base64::encode(secret.value.expose_secret());
    file.write(&secret.as_bytes()).map_err(|_| {
        TokenError::UnexpectedError
    })?;
    Ok(())
}

#[instrument(
    name = "Reading of secret from file",
    err
)]
fn read_secret_from_file() -> Result<SecretKey, TokenError> {
    let result = fs::read_to_string("token_secret.txt").map_err(|_| {
        TokenError::UnexpectedError
    })?;
    decode_of_secret(Secret::new(result))
}

#[instrument(
    name = "Decoding of secret",
    err
)]
fn decode_of_secret(result: Secret<String>) -> Result<SecretKey, TokenError> {
    let mut decoded_result: [u8; 64] = [0; 64];
    base64::decode_config_slice(
        &result.expose_secret(),
        base64::STANDARD,
        &mut decoded_result
    ).map_err( |_| {
        TokenError::UnexpectedError
    })?;

    if decoded_result.len() == 64 {
        Ok(SecretKey::new(Secret::new(decoded_result)))
    } else {
        Err(TokenError::UnexpectedError)
    }
}
