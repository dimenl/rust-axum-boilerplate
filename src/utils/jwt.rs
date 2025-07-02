use crate::types::error_types::AppError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

pub fn encode_jwt(user_id: Uuid) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims {
        sub: user_id.to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"secret"),
    )
}

pub fn decode_jwt(token: &str) -> Result<Uuid, AppError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(b"secret"),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized)?;
    Uuid::parse_str(&data.claims.sub).map_err(|_| AppError::Unauthorized)
}
