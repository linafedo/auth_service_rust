use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    token: String,
}

impl From<String> for LoginResponse {
    fn from(s: String) -> Self {
        LoginResponse { token: s }
    }
}