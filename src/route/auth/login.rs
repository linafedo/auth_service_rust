use actix_web::{Responder, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use tracing;
use tracing::{Instrument, instrument};
use secrecy::{Secret, ExposeSecret};

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    login: String,
    password: Secret<String>,
}

#[instrument(
    name = "Adding a new user",
    skip(form, pg_pool),
    fields(user_login = form.login)
)]
pub async fn registration(
    form: web::Form<AuthRequest>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {
    if form.password.expose_secret().len() < 6 {
        // todo - handle error
        return HttpResponse::SeeOther().finish()
    }
    match insert_user(form, pg_pool).await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[tracing::instrument(
name = "Saving new user in the database",
skip(form, pg_pool)
)]
pub async fn insert_user(
    form: web::Form<AuthRequest>,
    pg_pool: web::Data<PgPool>
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, login, password) VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        form.login,
        form.password.expose_secret())
        .execute(pg_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

pub async fn authentication(_from: web::Form<AuthRequest>) -> impl Responder {
    println!("AUTHENTICATION: Passed user login: - {}", _from.login);
    HttpResponse::Ok()
}