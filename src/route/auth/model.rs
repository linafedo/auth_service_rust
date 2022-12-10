use uuid::Uuid;
use crate::domain::user::model::PasswordData;

#[derive(serde::Deserialize)]
pub struct AuthData {
    login: String,
    password: String,
}

impl AuthData {
    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }
}

pub struct AuthUser {
    id: Uuid,
    login: String,
    password_hash: String,
    salt: String,
}

impl AuthUser {

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn new(id: Uuid, login: String, password_hash: String, salt: String) -> Self {
        AuthUser{ id, login, password_hash, salt }
    }

    pub fn get_password_hash(&self) -> &String {
        &self.password_hash
    }

    pub fn get_salt(&self) -> &String {
        &self.salt
    }
}