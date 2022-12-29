use utoipa::ToSchema;
use utoipa::{IntoParams};

#[derive(serde::Serialize, ToSchema)]
#[schema(example = json!(AuthResponse::example()))]
pub struct AuthResponse {
    /// User id
    id: String,
    /// User auth token
    token: String
}

impl AuthResponse {
    pub fn new(id: String, token: String) -> Self {
        AuthResponse{id, token}
    }
}

impl AuthResponse {
    fn example() -> Self {
        AuthResponse::new(
            "4a0f3fee-f4a3-4b8c-840b-5ec265db300b".to_string(),
            "eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNGEwZjNmZWUtZjRhMy00YjhjLTg0MGItNWVjMjY1ZGIzMDBiIiwiZXhwaXJhdGlvbl90aW1lc3RhbXAiOjE2NzI0MjEwOTJ9.cGMjzUOwlHZk2vjQFoDJI4C2kW1mV5DoleihL7wZc2I".to_string()
        )
    }
}