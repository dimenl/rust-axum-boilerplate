// Authentication related types

#[derive(Debug, serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
