use secrecy::{ExposeSecret, Secret};
use crate::route::dto::auth_data::AuthData;
use crate::route::registration::error::RegistrationError;
use crate::domain::user::user_data::{PasswordData, Password, Login};
use crate::repository::password::generate;

pub struct NewUser {
    pub login: Login,
    pub password: Password,
    pub password_data: PasswordData,
}

impl NewUser {
    pub fn new(
        login: Login,
        password: Password,
        password_data: PasswordData) -> Self
    {
        Self { login, password, password_data }
    }
}

impl TryFrom<AuthData> for NewUser {
    type Error = RegistrationError;

    fn try_from(value: AuthData) -> Result<Self, Self::Error> {
        let login = Login::parse(
            Secret::new(value.login.expose_secret().clone())
        )?;
        let password = Password::parse(
            Secret::new(value.password.expose_secret().clone())
        )?;
        let password_data = generate(
            Secret::new(value.password.expose_secret().clone())
        )?;
        Ok(Self { login, password, password_data })
    }
}