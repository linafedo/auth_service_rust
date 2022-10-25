use rocket::*;
use rocket_contrib::json::{Json, JsonValue};

use crate::handlers::{login, register};
use crate::handlers::login::{LoginResult, LoginError};
use crate::handlers::register::{RegistrationResult, RegistrationError};

use crate::routes::routes_object::error_response::{
    ERROR_USER_NOT_FOUND, ERROR_UNKNOWN, ERROR_WRONG_REQUEST, ERROR_WEAK_PASSWORD, ERROR_ALREADY_REGISTERED
};
use crate::routes::routes_object::auth_request::AuthRequest;
use crate::routes::routes_object::login_response::LoginResponse;
use crate::routes::routes_object::error_response::ErrorResponse;
use crate::routes;

use crate::database::DatabaseConnection;

#[post("/login", format = "json", data = "<login_request>")]
pub fn login<'r>(
    login_request: Option<Json<AuthRequest>>,
    db: DatabaseConnection
) -> Result<Json<LoginResponse>, ErrorResponse<'r>> {
    let login_result = login_request.map(|r|
        login::login(r.password, r.password, db)
    );
    return match login_result {
        Some(LoginResult::Success(token)) => {
            let login_response = LoginResponse::from(token);
            let json_response = Json(login_response);
            Result::Ok(json_response)
        }
        Some(LoginResult::Failed(error)) => {
            return match error {
                LoginError::NotFound => Result::Err(ERROR_USER_NOT_FOUND),
                LoginError::Other => Result::Err(ERROR_UNKNOWN),
            }
        }
        _ => Result::Err(ERROR_UNKNOWN),
    }
}

#[post("/registration", format = "json", data = "<registration_request>")]
pub fn registration<'r>(
    registration_request: Option<Json<AuthRequest>>,
    db: DatabaseConnection
) -> Result<(), ErrorResponse<'r>> {
    let registration_result = registration_request.map(|r|
        register::registration(r.login, r.password, db)
    );
    match registration_result {
        Some(RegistrationResult::Success) => Result::Ok(()),
        Some(RegistrationResult::Failed(error)) => {
            match error {
                RegistrationError::WeakPassword => Result::Err(ERROR_WEAK_PASSWORD),
                RegistrationError::UserExist => Result::Err(ERROR_ALREADY_REGISTERED),
                RegistrationError::Other => Result::Err(ERROR_UNKNOWN),
            }
        }
        _ => Result::Err(ERROR_UNKNOWN),
    }
}
