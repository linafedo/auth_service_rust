use crate::route::dto::auth_data::AuthData;
use crate::domain::user::auth_user::AuthUser;
use sqlx::{PgPool};
use actix_web::web;
use secrecy::{ExposeSecret, Secret};
use tracing::instrument;

#[instrument(
    name = "Check user in the database",
    skip(user, pg_pool),
    err
)]
pub async fn check_user(
    user: &AuthData,
    pg_pool: web::Data<PgPool>) -> Result<AuthUser, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, login, password_hash, salt FROM users WHERE login = $1
        "#,
        user.login.expose_secret(),
    )
        .fetch_one(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(AuthUser::new(
        result.id,
        Secret::new(result.login),
        Secret::new(result.password_hash),
        Secret::new(result.salt)
    ))
}