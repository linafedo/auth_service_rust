use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
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