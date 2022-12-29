use crate::route::dto::auth_data::AuthData;
use crate::route::registration::error::RegistrationError;
use crate::route::dto::error::ResponseError;
use crate::domain::user::new_user::NewUser;
use crate::repository::registration::insert_user;
use actix_web::{HttpResponse, web};
use actix_web::http::header::ContentType;
use sqlx::{Error, PgPool};
use tracing::{instrument};
use utoipa;
use serde_json::json;
use anyhow::Context;

/// Registration new user
#[utoipa::path(
    post,
    path = "/auth_service/v1/registration",
    request_body = AuthData,
    responses(
        (status = 200),
        (status = 409, body = ResponseError, example = json!(RegistrationError::password_not_correct_error_example())),
        (status = 400, body = ResponseError, example = json!(RegistrationError::user_exist_error_example())),
        (status = 500, body = ResponseError, example = json!(RegistrationError::unexpected_error_example()))
    ),
    tag = "Auth API",
)]
#[instrument(
    name = "Adding a new user",
    skip(form, pg_pool),
    err
)]
pub async fn registration(
    form: web::Json<AuthData>,
    pg_pool: web::Data<PgPool>
) -> Result<HttpResponse, RegistrationError> {
    let new_user = NewUser::try_from(form.0)?;

    insert_user(&new_user, pg_pool)
        .await
        .map_err( |e| {
            match e {
                Error::Database(dbe)
                if dbe.constraint() == Some("users_login_key") => {
                    RegistrationError::AlreadyExist
                }
                _ => {
                    RegistrationError::UnexpectedError
                }
            }
        })?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json().essence_str())
        .finish()
    )}