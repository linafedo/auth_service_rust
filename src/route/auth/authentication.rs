use crate::domain::user::model::PasswordData;
use crate::route::auth::error;
use crate::route::auth::model::{AuthUser, AuthData};
use crate::token_manager::token;

use actix_web::{HttpResponse, web};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use sqlx::{PgPool};
use tracing::instrument;

#[instrument(
    name = "User authentication",
    skip(form, pg_pool),
    fields(user_login = form.get_login())
)]
pub async fn authentication(
    form: web::Form<AuthData>,
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

    let mut response = HttpResponse::build(StatusCode::OK);
    let header = HeaderValue::from_str(token.as_str())
        .map_err(|e|
            error::AuthenticationError::UnexpectedError(anyhow::Error::from(e))
        )?;

    response.insert_header(("token", header));
    Ok(response.finish())
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