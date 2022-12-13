use uuid::Uuid;

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