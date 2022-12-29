use secrecy::Secret;
use utoipa::ToSchema;
use utoipa::{IntoParams};

/// Login should not contains symbols:
/// '/', '(', ')', '"', '<', '>', '\\', '{', '}', '-'.
/// Min login length should be 3 symbols.
/// Min password length should be 6 symbols.
#[derive(serde::Deserialize, ToSchema, Clone, IntoParams)]
#[schema(example = json!({"login": "Joni", "password": 123456}))]
pub struct AuthData {
    #[schema(value_type = String)]
    pub login: Secret<String>,
    #[schema(value_type = String)]
    pub password: Secret<String>,
}