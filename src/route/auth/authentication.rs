use std::arch::asm;
use actix_web::{HttpResponse, ResponseError, web};
use actix_web::http::StatusCode;
use sqlx::{Error, PgPool};
use tracing::{Instrument, instrument};
use uuid::Uuid;
use crate::route::domain::PasswordData;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Password is not correct")]
    PasswordNotCorrect,
    #[error("User not exist")]
    UserNotExist,
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticationError::PasswordNotCorrect => StatusCode::BAD_REQUEST,
            AuthenticationError::UserNotExist => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AuthData {
    login: String,
    password: String,
}

pub struct AuthUser {
    id: Uuid,
    login: String,
    password_hash: String,
    salt: String,
}

#[instrument(
    name = "User authentication",
    skip(form, pg_pool),
    fields(user_login = form.login)
)]

pub async fn authentication(
    form: web::Form<AuthData>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {
    match check_user(&form.0, pg_pool).await {
        Ok(user) => {
            match PasswordData::check_password(
                form.password.as_str(),
                user.salt.as_str(),
                user.password_hash.as_str()

            ) {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::from_error(AuthenticationError::PasswordNotCorrect)
            }
        },
        Err(e) => {
            match e {
                Error::RowNotFound => {
                    HttpResponse::from_error(AuthenticationError::UserNotExist)
                }
                _ => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[tracing::instrument(
    name = "Check user in the database",
    skip(user, pg_pool)
)]
async fn check_user(
    user: &AuthData,
    pg_pool: web::Data<PgPool>) -> Result<AuthUser, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, login, password_hash, salt FROM users WHERE login = $1
        "#,
        user.login,
    )
        .fetch_one(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    let user = AuthUser{
        id: result.id,
        login: result.login,
        password_hash: result.password_hash,
        salt: result.salt
    };
    Ok(user)
}