use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct APIError {
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("{0}")]
    Message(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Message(_) => StatusCode::BAD_REQUEST,
        };
        let api_error = APIError {
            message: self.to_string(),
        };
        (status, Json(api_error)).into_response()
    }
}
