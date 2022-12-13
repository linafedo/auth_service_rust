use crate::domain::user::user_data::PasswordData;
use crate::route::auth::error;
use crate::domain::user::auth_user::AuthUser;
use crate::route::dto::auth_data::AuthData;
use crate::route::dto::auth_response::AuthResponse;
use crate::auth_token::token;

use actix_web::{HttpResponse, web};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use sqlx::{PgPool};
use tracing::instrument;
use utoipa;

#[utoipa::path(
    get,
    path = "/api/v1/authentication",
    request_body = AuthData,
    responses(
        (status = 200, body = AuthResponse),
        (status = 409, description = "Password is not correct"),
        (status = 400, description = "User not exist")
    ),
)]
#[instrument(
    name = "User authentication",
    skip(form, pg_pool),
    fields(user_login = form.get_login())
)]
pub async fn authentication(
    form: web::Json<AuthData>,
    pg_pool: web::Data<PgPool>
) -> Result<HttpResponse, error::AuthenticationError> {
    let user = check_user(&form.0, pg_pool)
        .await
        .map_err(|e| {
        match e {
            sqlx::Error::RowNotFound => { error::AuthenticationError::UserNotExist }
            _ => { error::AuthenticationError::UnexpectedError(anyhow::Error::from(e)) }
        }
    })?;
    PasswordData::check_password(
        form.get_password(),
        user.get_salt(),
        user.get_password_hash()

    ).map_err(|_| {
        error::AuthenticationError::PasswordNotCorrect
    })?;

    let token = token::new_token(user.get_id().to_string().as_str())
        .map_err(|e|
            error::AuthenticationError::UnexpectedError(anyhow::Error::from(e))
        )?;

    let response = AuthResponse::new(user.get_id().to_string(), token);
    Ok(HttpResponse::Created().json(response))
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