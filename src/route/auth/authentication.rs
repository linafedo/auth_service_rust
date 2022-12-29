use crate::route::auth::error::AuthenticationError;
use crate::route::dto::auth_data::AuthData;
use crate::route::dto::auth_response::AuthResponse;
use crate::auth_token::token;
use crate::repository::authentication::check_user;
use crate::configuration::{Config, Authentication};
use crate::repository::password_data::password::check_password;

use actix_web::{HttpResponse, web};
use secrecy::ExposeSecret;
use sqlx::{PgPool};
use tracing::instrument;
use utoipa;

/// Auth user with login and password
#[utoipa::path(
    post,
    path = "/auth_service/v1/authentication",
    request_body = AuthData,
    responses(
        (status = 200, body = AuthResponse),
        (status = 409, body = ResponseError, example = json!(AuthenticationError::password_not_correct_error_example())),
        (status = 400, body = ResponseError, example = json!(AuthenticationError::user_not_exist_error_example())),
        (status = 500, body = ResponseError, example = json!(AuthenticationError::unexpected_error_example()))
    ),
    tag = "Auth API",
)]
#[instrument(
    name = "User authentication",
    skip(form, pg_pool),
    err
)]
pub async fn authentication(
    form: web::Json<AuthData>,
    pg_pool: web::Data<PgPool>,
    data: web::Data<Authentication>
) -> Result<HttpResponse, AuthenticationError> {
    let user = check_user(&form.0.clone(), pg_pool)
        .await
        .map_err(|e| {
        match e {
            sqlx::Error::RowNotFound => { AuthenticationError::UserNotExist }
            _ => { AuthenticationError::UnexpectedError }
        }
    })?;
    check_password(
        form.password.clone(),
        user.salt.clone(),
        user.password_hash.clone()

    ).map_err(|_| {
        AuthenticationError::PasswordNotCorrect
    })?;

    let token = token::new_token(
        user.id,
        data.token_duration_in_sec
    )
        .map_err(|_|
            AuthenticationError::UnexpectedError
        )?;

    let response = AuthResponse::new(user.id.to_string(), token);
    Ok(HttpResponse::Created().json(response))
}