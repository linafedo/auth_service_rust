use actix_web::{Responder, HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    login: String,
    password: String,
}

pub async fn registration(_from: web::Form<AuthRequest>) -> impl Responder {
    println!("REGISTRATION: Passed user login: - {}", _from.login);
    HttpResponse::Ok()
}

pub async fn authentication(_from: web::Form<AuthRequest>) -> impl Responder {
    println!("AUTHENTICATION: Passed user login: - {}", _from.login);
    HttpResponse::Ok()
}