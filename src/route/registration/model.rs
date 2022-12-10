use crate::domain::user::model::{PasswordData, UserLogin, UserPassword};
use crate::route::registration::error::RegistrationError;

#[derive(serde::Deserialize)]
pub struct FormData {
    login: String,
    password: String,
}

impl FormData {
    pub fn get_login(&self) -> &str {
        &self.login
    }
}

impl TryFrom<FormData> for NewUser {
    type Error = RegistrationError;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let login = UserLogin::parse(value.login.clone())?;
        let password = UserPassword::parse(value.password.clone())?;
        let password_data = PasswordData::generate(password.expose_secret())?;
        Ok(Self { login, password, password_data })
    }
}

pub struct NewUser {
    pub login: UserLogin,
    pub password: UserPassword,
    pub password_data: PasswordData,
}