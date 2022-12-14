use crate::route::dto::auth_data::AuthData;
use crate::route::registration::error::RegistrationError;
use crate::domain::user::new_user::NewUser;
use crate::route::dto::error::ResponseError;

use actix_web::{HttpResponse, web};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use tracing::{instrument};
use utoipa;
use serde_json::json;

#[utoipa::path(
    post,
    path = "/api/v1/registration",
    request_body = AuthData,
    responses(
        (status = 200),
        (status = 409, body = ResponseError, example = json!(RegistrationError::password_not_correct_error_example())),
        (status = 400, body = ResponseError, example = json!(RegistrationError::user_exist_error_example())),
        (status = 500, body = ResponseError, example = json!(ResponseError::internal_error_example()))
    ),
)]
#[instrument(
    name = "Adding a new user",
    skip(form, pg_pool),
    fields(user_login = form.get_login())
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
                    RegistrationError::UnexpectedError(anyhow::Error::from(e))
                }
            }
        })?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new user in the database",
    skip(user, pg_pool)
)]
pub async fn insert_user(
    user: &NewUser,
    pg_pool: web::Data<PgPool>
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, login, salt, password_hash)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user.login.as_ref(),
        user.password_data.get_salt(),
        user.password_data.get_password_hash()
    )
        .execute(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}