use secrecy::{ExposeSecret, Secret};
use crate::route::dto::auth_data::AuthData;
use crate::route::registration::error::RegistrationError;
use crate::domain::user::user_data::{PasswordData, Password, Login};
use crate::repository::password_data::password::generate;
use crate::domain::user::error::Error;

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
    type Error = anyhow::Error;

    fn try_from(value: AuthData) -> Result<Self, anyhow::Error> {
        let login = Login::parse(
            Secret::new(value.login.expose_secret().clone())
        )
            .map_err(|e| anyhow::Error::new(e))?;

        let password = Password::parse(
            Secret::new(value.password.expose_secret().clone())
        )
            .map_err(|e| anyhow::Error::new(e))?;

        let password_data = generate(
            Secret::new(value.password.expose_secret().clone())
        )
            .map_err(|e| anyhow::Error::new(e))?;

        Ok(Self { login, password, password_data })
    }
}