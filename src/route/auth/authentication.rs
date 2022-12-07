use crate::route::registration::domain::PasswordData;
use crate::route::auth::error::AuthenticationError;
use crate::route::auth::model::{AuthUser, AuthData};

use std::arch::asm;
use actix_web::{HttpResponse, ResponseError, web};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use actix_web::web::to;
use sqlx::{Error, PgPool};
use tracing::{Instrument, instrument};
use uuid::Uuid;
use crate::token_manager::error::TokenManagerError;
use crate::token_manager::token;

#[instrument(
    name = "User authentication",
    skip(form, pg_pool),
    fields(user_login = form.get_login())
)]
pub async fn authentication(
    form: web::Form<AuthData>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {
    match check_user(&form.0, pg_pool).await {
        Ok(user) => {
            match PasswordData::check_password(
                form.get_password(),
                user.get_salt(),
                user.get_password_hash()

            ) {
                Ok(_) => {
                    let token = token::new_token(user.get_id().to_string().as_str());
                    if let Ok(token) = token {
                        let mut response = HttpResponse::build(StatusCode::OK);
                        let header = HeaderValue::from_str(token.as_str()).unwrap();
                        // TODO unwrap!!!!
                        response.insert_header(("token", header));
                        return response.finish()
                    } else {
                        // todo
                        HttpResponse::from_error(AuthenticationError::UserNotExist)
                    }

                },
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
        user.get_login(),
    )
        .fetch_one(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(AuthUser::new(result.id, result.login, result.password_hash, result.salt))
}