use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
    pub status_code: StatusCode,
    pub error_message: String,
    pub user_message: String,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!({
                "status_code": self.status_code.to_string(),
                "error_message": self.error_message,
                "user_message": self.user_message
            })),
        )
            .into_response()
    }
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

impl From<AppError> for APIError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Unauthorized => APIError {
                status_code: StatusCode::UNAUTHORIZED,
                error_message: "Unauthorized".into(),
                user_message: "Unauthorized".into(),
            },
            AppError::NotFound => APIError {
                status_code: StatusCode::NOT_FOUND,
                error_message: "Not Found".into(),
                user_message: "Not Found".into(),
            },
            AppError::Message(msg) => APIError {
                status_code: StatusCode::BAD_REQUEST,
                error_message: msg.clone(),
                user_message: msg,
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        APIError::from(self).into_response()
    }
}
