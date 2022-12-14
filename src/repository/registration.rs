use crate::domain::user::new_user::NewUser;
use sqlx::{Error, PgPool};
use actix_web::web;
use secrecy::ExposeSecret;
use uuid::Uuid;

#[tracing::instrument(
    name = "Saving new user in the database",
    skip(user, pg_pool),
    err
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
        user.password_data.salt.expose_secret(),
        user.password_data.password_hash.expose_secret()
    )
        .execute(pg_pool.get_ref())
        .await?;
    Ok(())
}