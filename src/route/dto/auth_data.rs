use secrecy::Secret;
use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema, Clone)]
pub struct AuthData {
    pub login: Secret<String>,
    pub password: Secret<String>,
}