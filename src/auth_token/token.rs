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


#[derive(Default, Deserialize, Serialize)]
struct TokenData {
    user_id: String,
    expiration_timestamp: i64,
}

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
    let signed_token = unsigned_token.sign_with_key(&secret_key).map_err(|e| {
        tracing::error!("Error signing token with secret key {}", e.to_string());
        TokenError::UnexpectedError
    })?;
    Ok(signed_token.into())
}


pub fn verify_token(token: &str) -> Result<(), TokenError> {
    match read_secret_from_file() {
        Ok(secret) => {
            let generated_key: Hmac<Sha256> = Hmac::new_from_slice(&secret).map_err(|e| {
                tracing::error!("Error generating new secret key {}", e.to_string());
                TokenError::UnexpectedError
            })?;
            VerifyWithKey::verify_with_key(token, &generated_key).map_err(|_| {
                tracing::error!("Error verify token");
                TokenError::VerifyTokenError
            })?;
            Ok(())
        }
        Err(_) => Err(TokenError::UnexpectedError)
    }
}

fn read_or_generate_secret_key() -> Result<Hmac<Sha256>, TokenError> {
    if let Ok(secret) = read_secret_from_file() {
        let key = Hmac::new_from_slice(&secret).map_err(|e| {
            tracing::error!("Error generating new secret key {}", e.to_string());
            TokenError::UnexpectedError
        })?;
        Ok(key)
    } else {
        Ok(create_and_save_secret_key()?)
    }
}

fn create_and_save_secret_key() -> Result<Hmac<Sha256>, TokenError> {
    let mut key: [u8; 64] = [0; 64];
    let mut default = OsRng::default();
    default.fill_bytes(&mut key);

    let generated_key: Hmac<Sha256> = Hmac::new_from_slice(&key).map_err(|e| {
        tracing::error!("Error generating new secret key {}", e.to_string());
        TokenError::UnexpectedError
    })?;
    save_secret(&key).map_err(|_| TokenError::UnexpectedError)?;
    Ok(generated_key)
}

fn save_secret(secret: &[u8]) -> Result<(), TokenError> {
    let mut file = fs::File::create_new("token_secret.txt").map_err(|e| {
        tracing::error!("Error creating file for secret {}", e.to_string());
        return TokenError::UnexpectedError
    })?;

    let secret = base64::encode(secret);
    file.write(&secret.as_bytes()).map_err(|e| {
        tracing::error!("Write secret to file failed {}", e.to_string());
        return TokenError::UnexpectedError
    })?;
    Ok(())
}

fn read_secret_from_file() -> Result<[u8; 64], TokenError> {
    let result = fs::read_to_string("token_secret.txt").map_err(|e| {
        tracing::error!("Error decoding secret {}", e.to_string());
        TokenError::UnexpectedError
    })?;
    handle_result(result)
}

fn handle_result(result: String) -> Result<[u8; 64], TokenError> {
    let mut decoded_result: [u8; 64] = [0; 64];
    base64::decode_config_slice(
        &result,
        base64::STANDARD,
        &mut decoded_result
    ).map_err( |e| {
        tracing::error!("Error decoding secret {}", e.to_string());
        TokenError::UnexpectedError
    })?;

    if decoded_result.len() == 64 {
        Ok(decoded_result)
    } else {
        tracing::error!("File with secret does not match the length");
        Err(TokenError::UnexpectedError)
    }
}
