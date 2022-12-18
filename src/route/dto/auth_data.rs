use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
pub struct AuthData {
    pub login: String,
    pub password: String,
}