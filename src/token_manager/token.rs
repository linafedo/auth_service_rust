use crate::token_manager::error::TokenManagerError;
use crate::token_manager::token::TokenError::{
    DecodeSecretError, FileWithSecretNotFound, WrongFileLength,
};
use crate::token_manager::token;

use jwt::{SignWithKey, VerifyWithKey, Header, Token, ToBase64};
use chrono::{Utc, Duration};
use uuid::Uuid;
use anyhow::Error;
use hmac::digest::KeyInit;
use hmac::Hmac;
use serde::{Serialize, Deserialize};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use tracing::{Instrument, instrument};
use std::fs;
use actix_web::body::MessageBody;
use actix_web::web::to;
use base64::Config;
use std::collections::BTreeMap;


#[derive(Default, Deserialize, Serialize)]
struct Custom {
    user_id: String,
    expiration: i64,
}

pub fn new_token(user_id: &str) -> Result<String, Error> {
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
        TokenManagerError::SignWithKeyError
    })?;
    println!("verify token result- {:?}", verify_token(signed_token.as_str()));
    Ok(signed_token.into())
}


pub fn verify_token(token: &str) -> Result<(), ()> {
    match read_secret_from_file() {
        Ok(secret) => {
            let generated_key: Hmac<Sha256> = Hmac::new_from_slice(&secret).unwrap();
            let token: Token<Header, Custom, _> = VerifyWithKey::verify_with_key(token, &generated_key).unwrap();
            Ok(())
        }
        Err(e) => { Err(()) }
    }
}

fn save_secret(secret: &[u8]) -> Result<(), Error> {
    let mut file = fs::File::create_new("token_secret.txt").map_err(|e| {
        tracing::error!("Error creating file for secret {}", e.to_string());
        return e;
    })?;
    let secret = base64::encode(secret);
    match file.write(&secret.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Write secret to file failed");
            Err(Error::from(e))
        }
    }
}

fn create_and_save_secret_key() -> Result<Hmac<Sha256>, Error> {
    let mut key: [u8; 64] = [0; 64];
    let mut default = OsRng::default();
    default.fill_bytes(&mut key);

    let generated_key: Hmac<Sha256> = Hmac::new_from_slice(&key).map_err(|e| {
        tracing::error!("Error generating secret key - {}", e.to_string());
        TokenManagerError::SignWithKeyError
    })?;
    match save_secret(&key) {
        Ok(_) => Ok(generated_key),
        Err(e) => Err(Error::from(e))
    }
}

fn read_or_generate_secret_key() -> Result<Hmac<Sha256>, Error> {
    match read_secret_from_file() {
        Ok(secret) => {
            let key = Hmac::new_from_slice(&secret)?;
            Ok(key)
        }
        Err(e) => match e {
            TokenError::FileWithSecretNotFound => {
                let secret_key = create_and_save_secret_key()?;
                Ok(secret_key)
            }
            _ => { Err(Error::from(TokenManagerError::SignWithKeyError)) }
        }
    }
}

enum TokenError {
    WrongFileLength,
    FileWithSecretNotFound,
    DecodeSecretError,
}

fn read_secret_from_file() -> Result<[u8; 64], TokenError> {
    match fs::read_to_string("token_secret.txt") {
        Ok(result) => {
            let mut decoded_result: [u8; 64] = [0; 64];
            match base64::decode_config_slice(
                &result,
                base64::STANDARD,
                &mut decoded_result)
            {
                Ok(_) => {
                    if decoded_result.len() == 64 {
                        Ok(decoded_result)
                    } else {
                        tracing::error!("File with secret does not match the length");
                        Err(TokenError::WrongFileLength)
                    }
                }
                Err(e) => {
                    tracing::error!("File with secret does not match the length");
                    Err(TokenError::DecodeSecretError)
                }
            }
        }
        Err(e) => {
            tracing::error!("Error reading secret key - {}", e.to_string());
            Err(TokenError::FileWithSecretNotFound)
        }
    }
}
