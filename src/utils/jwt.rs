use crate::types::error_types::AppError;
use crate::utils::{ACCESS_TOKEN, TOKEN_EXPIRATION_SECS};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn encode_jwt(user_id: Uuid) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now().timestamp() as usize) + *TOKEN_EXPIRATION_SECS,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ACCESS_TOKEN.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Uuid, AppError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(ACCESS_TOKEN.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized)?;
    if data.claims.exp < Utc::now().timestamp() as usize {
        return Err(AppError::Unauthorized);
    }
    Uuid::parse_str(&data.claims.sub).map_err(|_| AppError::Unauthorized)
}
