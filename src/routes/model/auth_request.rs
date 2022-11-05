use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthRequest<'a> {
    pub login: &'a str,
    pub password: &'a str
}