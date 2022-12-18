use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

pub struct AuthUser {
    pub id: Uuid,
    pub login: String,
    pub password_hash: String,
    salt: Secret<String>,
}

impl AuthUser {
    pub fn new(id: Uuid, login: String, password_hash: String, salt: String) -> Self {
        AuthUser{ id, login, password_hash, salt: Secret::new(salt) }
    }

    pub fn get_salt(&self) -> &str {
        &self.salt.expose_secret()
    }
}