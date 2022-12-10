use crate::token_manager::error::TokenError;

use jwt::{SignWithKey, VerifyWithKey, Header, Token};
use chrono::{Duration};
use hmac::digest::KeyInit;
use hmac::Hmac;
use serde::{Serialize, Deserialize};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use std::io::Write;
use std::fs;


#[derive(Default, Deserialize, Serialize)]
struct Custom {
    user_id: String,
    expiration: i64,
}

pub fn new_token(user_id: &str) -> Result<String, TokenError> {
    // header
    let header: Header = Default::default();
    // claims
    let expiration_time = chrono::Utc::now() + Duration::days(1);
    let claims = Custom {
        user_id: user_id.into(),
        expiration: expiration_time.timestamp(),
    };
    // key
    let secret_key = read_or_generate_secret_key()?;

    let unsigned_token = Token::new(header, claims);
    let signed_token = unsigned_token.sign_with_key(&secret_key).map_err(|e| {
        tracing::error!("Error signing token with secret key {}", e.to_string());
        TokenError::SignTokenError(e.to_string())
    })?;
    // TODO
    println!("verify token result- {:?}", verify_token(signed_token.as_str())?);
    Ok(signed_token.into())
}


pub fn verify_token(token: &str) -> Result<(), TokenError> {
    match read_secret_from_file() {
        Ok(secret) => {
            let generated_key: Hmac<Sha256> = Hmac::new_from_slice(&secret).map_err(|e| {
                tracing::error!("Error generating new secret key {}", e.to_string());
                TokenError::GenerateKeyError(e.to_string())
            })?;
            let _token: Token<Header, Custom, _> = VerifyWithKey::verify_with_key(token, &generated_key).map_err(|_| {
                tracing::error!("Error verify token");
                TokenError::VerifyTokenError
            })?;
            Ok(())
        }
        Err(e) => { Err(e) }
    }
}

fn read_or_generate_secret_key() -> Result<Hmac<Sha256>, TokenError> {
    match read_secret_from_file() {
        Ok(secret) => {
            let key = Hmac::new_from_slice(&secret).map_err(|e| {
                tracing::error!("Error generating new secret key {}", e.to_string());
                TokenError::GenerateKeyError(e.to_string())
            })?;
            Ok(key)
        }
        Err(e) => match e {
            TokenError::FileWithSecretNotFound(_) => {
                let secret_key = create_and_save_secret_key()?;
                Ok(secret_key)
            }
            _ => { Err(e) }
        }
    }
}

fn create_and_save_secret_key() -> Result<Hmac<Sha256>, TokenError> {
    let mut key: [u8; 64] = [0; 64];
    let mut default = OsRng::default();
    default.fill_bytes(&mut key);

    let generated_key: Hmac<Sha256> = Hmac::new_from_slice(&key).map_err(|e| {
        tracing::error!("Error generating new secret key {}", e.to_string());
        TokenError::GenerateKeyError(e.to_string())
    })?;

    match save_secret(&key) {
        Ok(_) => Ok(generated_key),
        Err(e) => Err(e)
    }
}

fn save_secret(secret: &[u8]) -> Result<(), TokenError> {
    let mut file = fs::File::create_new("token_secret.txt").map_err(|e| {
        tracing::error!("Error creating file for secret {}", e.to_string());
        return TokenError::CreateFileForSecretError(e.to_string())
    })?;

    let secret = base64::encode(secret);
    match file.write(&secret.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Write secret to file failed {}", e.to_string());
            return Err(TokenError::WriteSecretToFileError(e.to_string()))
        }
    }
}

fn read_secret_from_file() -> Result<[u8; 64], TokenError> {
    match fs::read_to_string("token_secret.txt") {
        Ok(result) => {
            return handle_result(result)
        }
        Err(e) => {
            tracing::error!("Error decoding secret {}", e.to_string());
            Err(TokenError::FileWithSecretNotFound(e.to_string()))
        }
    }
}

fn handle_result(result: String) -> Result<[u8; 64], TokenError> {
    let mut decoded_result: [u8; 64] = [0; 64];
    match base64::decode_config_slice(
        &result,
        base64::STANDARD,
        &mut decoded_result
    ) {
        Ok(_) => {
            if decoded_result.len() == 64 {
                Ok(decoded_result)
            } else {
                tracing::error!("File with secret does not match the length");
                Err(TokenError::WrongFileLength)
            }
        }
        Err(e) => {
            tracing::error!("Error decoding secret {}", e.to_string());
            Err(TokenError::DecodeSecretError(e.to_string()))
        }
    }
}
