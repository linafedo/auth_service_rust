use crate::route::registration::model::{FormData, NewUser};
use crate::route::registration::error::RegistrationError;
use crate::route::registration::domain::PasswordData;

use std::fmt::{Display, Formatter};
use actix_web::{HttpResponse, web};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use tracing::{Instrument, instrument};

#[instrument(
    name = "Adding a new user",
    skip(form, pg_pool),
    fields(user_login = form.get_login())
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