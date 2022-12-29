use secrecy::{ExposeSecret, Secret};
use utoipa::ToSchema;
use utoipa::{IntoParams};
use serde::{Serialize, Deserialize, Serializer};

/// Login should not contains symbols:
/// '/', '(', ')', '"', '<', '>', '\\', '{', '}', '-'.
/// Min login length should be 3 symbols.
/// Min password length should be 6 symbols.
#[derive(Deserialize, Serialize, ToSchema, Clone, IntoParams)]
#[schema(example = json!(AuthData::example()))]
pub struct AuthData {
    #[serde(serialize_with = "serialize_secret")]
    pub login: Secret<String>,
    #[serde(serialize_with = "serialize_secret")]
    pub password: Secret<String>,
}

impl AuthData {
    fn example() -> Self {
        AuthData{
            login: Secret::new("Joni".to_string()),
            password: Secret::new("123456".to_string())
        }
    }
}

fn serialize_secret<S>(x: &Secret<String>, s: S)
    -> Result<S::Ok, S::Error>
    where S: Serializer {
    s.serialize_str(x.expose_secret())
}