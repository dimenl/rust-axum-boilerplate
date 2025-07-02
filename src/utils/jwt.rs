use jsonwebtoken::{encode, EncodingKey, Header};
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
