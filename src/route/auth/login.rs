use actix_web::{Responder, HttpResponse, web};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    login: String,
    password: String,
}

pub async fn registration(
    form: web::Form<AuthRequest>,
    pg_pool: web::Data<PgPool>
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO users (id, login, password) VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        form.login,
        form.password
    )
        .execute(pg_pool.get_ref())
        .await {
        Ok(_) => {
            println!("REGISTRATION: success for login:- {}", form.login);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            println!("REGISTRATION: Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn authentication(_from: web::Form<AuthRequest>) -> impl Responder {
    println!("AUTHENTICATION: Passed user login: - {}", _from.login);
    HttpResponse::Ok()
}