use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

pub struct AuthUser {
    pub id: Uuid,
    pub login: Secret<String>,
    pub password_hash: Secret<String>,
    pub salt: Secret<String>,
}

impl AuthUser {
    pub fn new(
        id: Uuid,
        login: Secret<String>,
        password_hash: Secret<String>,
        salt: Secret<String>
    ) -> Self {
        AuthUser{ id, login, password_hash, salt }
    }
}