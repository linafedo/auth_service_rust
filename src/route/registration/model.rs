use crate::route::domain::{UserLogin, UserPassword};
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
        Ok(Self { login, password })
    }
}

pub struct NewUser {
    pub login: UserLogin,
    pub password: UserPassword,
}