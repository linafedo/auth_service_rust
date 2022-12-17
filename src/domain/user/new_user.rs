use crate::route::dto::auth_data::AuthData;
use crate::route::registration::error::RegistrationError;
use crate::domain::user::user_data::{PasswordData, UserPassword, UserLogin};
use crate::password_manager::manager::generate;
pub struct NewUser {
    pub login: UserLogin,
    pub password: UserPassword,
    pub password_data: PasswordData,
}

impl NewUser {
    pub fn new(
        login: UserLogin,
        password: UserPassword,
        password_data: PasswordData) -> Self
    {
        Self { login, password, password_data }
    }
}

impl TryFrom<AuthData> for NewUser {
    type Error = RegistrationError;

    fn try_from(value: AuthData) -> Result<Self, Self::Error> {
        let login = UserLogin::parse(value.get_login().clone())?;
        let password = UserPassword::parse(value.get_password().clone())?;
        let password_data = generate(password.expose_secret())?;
        Ok(Self { login, password, password_data })
    }
}