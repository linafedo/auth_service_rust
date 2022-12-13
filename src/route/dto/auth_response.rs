use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct AuthResponse {
    id: String,
    token: String
}

impl AuthResponse {
    pub fn new(id: String, token: String) -> Self {
        AuthResponse{id, token}
    }
}