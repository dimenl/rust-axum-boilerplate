use crate::types::error_types::AppError;
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::de::DeserializeOwned;

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S, Body> for CustomJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let (_, body) = req.into_parts();
        let bytes = match axum::body::to_bytes(body, usize::MAX).await {
            Ok(b) => b,
            Err(e) => {
                return Err(AppError::Message {
                    status_code: StatusCode::BAD_REQUEST,
                    error_message: e.to_string(),
                    user_message: Some("Failed to read request body".to_string()),
                });
            }
        };
        serde_json::from_slice::<T>(&bytes)
            .map(CustomJson)
            .map_err(|e| AppError::Message {
                status_code: StatusCode::BAD_REQUEST,
                error_message: e.to_string(),
                user_message: Some("Invalid request body".to_string()),
            })
    }
}
