use std::fmt::{Display, Formatter};
use std::ops::Deref;
use actix_web::{Responder, HttpResponse, web, ResponseError};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use tracing;
use tracing::{Instrument, instrument};
use secrecy::{Secret, ExposeSecret};
use anyhow;
use actix_web::http::StatusCode;
use once_cell::sync::Lazy;

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("Password must contain at least 6 characters")]
    PasswordNotCorrect,
    #[error("User already exists")]
    AlreadyExist,
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::PasswordNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::AlreadyExist => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    login: String,
    password: Secret<String>,
}

#[instrument(
    name = "Adding a new user",
    skip(form, pg_pool),
    fields(user_login = form.login)
)]
pub async fn registration(
    form: web::Form<AuthRequest>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {
    match validate_data(&form) {
        Ok(_) => { },
        Err(e) => return HttpResponse::from_error(e),
    }

    match insert_user(&form, pg_pool).await {
        Ok(_) => { HttpResponse::Ok().finish() },
        Err(e) => {
            match e {
                Error::Database(dbe)
                if dbe.constraint() == Some("users_login_key") => {
                    HttpResponse::from_error(RegistrationError::AlreadyExist)
                }
                _ => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

fn validate_data(form: &web::Form<AuthRequest>) -> Result<(), RegistrationError> {
    if form.password.expose_secret().len() < 6 { return Err(RegistrationError::PasswordNotCorrect) }
    Ok(())
}

#[tracing::instrument(
    name = "Saving new user in the database",
    skip(form, pg_pool)
)]
pub async fn insert_user(
    form: &web::Form<AuthRequest>,
    pg_pool: web::Data<PgPool>
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, login, password)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        form.login,
        form.password.expose_secret())
        .execute(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

pub async fn authentication(_from: web::Form<AuthRequest>) -> impl Responder {
    println!("AUTHENTICATION: Passed user login: - {}", _from.login);
    HttpResponse::Ok()
}