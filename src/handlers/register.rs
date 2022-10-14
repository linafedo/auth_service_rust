use crate::database::auth::create_user;
use crate::database::auth::create_user::CreateUser;

pub enum RegistrationError {
    UserExist,
    WeakPassword,
    Other
}

pub enum RegistrationResult {
    Success,
    Failed(RegistrationError)
}

pub fn registration(
    login: &str,
    password: &str,
    db: &mut diesel::PgConnection
) -> RegistrationResult {
    match create_user::create_user(db, login, password) {
        CreateUser::Ok => RegistrationResult::Success,
        CreateUser::WeakPassword => RegistrationResult::Failed(RegistrationError::WeakPassword),
        CreateUser::AlreadyExist => RegistrationResult::Failed(RegistrationError::UserExist),
        CreateUser::Other => RegistrationResult::Failed(RegistrationError::Other),
    }
}