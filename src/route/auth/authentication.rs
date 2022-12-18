use crate::route::auth::error;
use crate::route::dto::auth_data::AuthData;
use crate::route::dto::auth_response::AuthResponse;
use crate::auth_token::token;
use crate::repository::authentication::check_user;
use crate::configuration::Config;
use crate::password_manager::manager::check_password;

use actix_web::{HttpResponse, web};
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
    fields(user_login = form.login)
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
            _ => { error::AuthenticationError::UnexpectedError }
        }
    })?;
    check_password(
        form.password.as_str(),
        user.get_salt(),
        user.password_hash.as_str()

    ).map_err(|_| {
        error::AuthenticationError::PasswordNotCorrect
    })?;

    let config = Config::load().map_err(|_|
        error::AuthenticationError::UnexpectedError
    )?;

    let token = token::new_token(
        user.id,
        config.authentication.token_duration_in_days
    )
        .map_err(|_|
            error::AuthenticationError::UnexpectedError
        )?;

    let response = AuthResponse::new(user.id.to_string(), token);
    Ok(HttpResponse::Created().json(response))
}