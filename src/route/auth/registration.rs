use crate::route::domain;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use actix_web::{Responder, HttpResponse, web, ResponseError};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use tracing::{Instrument, instrument};
use secrecy::{Secret, ExposeSecret};
use actix_web::http::StatusCode;
use once_cell::sync::Lazy;
use tracing_actix_web::root_span_macro::private::http_flavor;
use crate::route::domain::{UserLogin, UserPassword};

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("Password must contain at least 6 characters")]
    PasswordNotCorrect,
    #[error("User already exists")]
    AlreadyExist,
    #[error("Login must contain from 3 to 256 characters")]
    LoginLengthIsWrong,
    #[error("Login should be contain only letters and numbers and start with a letter")]
    LoginIsNotCorrect,
    #[error("Login should be not empty")]
    LoginIsEmpty
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::PasswordNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::AlreadyExist => StatusCode::INTERNAL_SERVER_ERROR,
            RegistrationError::LoginLengthIsWrong => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsNotCorrect => StatusCode::BAD_REQUEST,
            RegistrationError::LoginIsEmpty => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    login: String,
    password: String,
}

impl TryFrom<FormData> for NewUser {
    type Error = RegistrationError;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let login = UserLogin::parse(value.login.clone())?;
        let password = UserPassword::parse(value.password.clone())?;
        Ok(Self { login, password })
    }
}

pub struct NewUser {
    pub login: UserLogin,
    pub password: UserPassword,
}

#[instrument(
    name = "Adding a new user",
    skip(form, pg_pool),
    fields(user_login = form.login)
)]
pub async fn registration(
    form: web::Form<FormData>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {
    let new_user= match NewUser::try_from(form.0) {
        Ok(new_user) => new_user,
        Err(e) => return HttpResponse::from_error(e)
    };

    match insert_user(&new_user, pg_pool).await {
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

#[tracing::instrument(
    name = "Saving new user in the database",
    skip(user, pg_pool)
)]
pub async fn insert_user(
    user: &NewUser,
    pg_pool: web::Data<PgPool>
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, login, password)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        user.login.as_ref(),
        user.password.as_ref()
    )
        .execute(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}