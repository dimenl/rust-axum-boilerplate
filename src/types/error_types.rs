use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
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
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("{error_message}")]
    Message {
        status_code: StatusCode,
        error_message: String,
        user_message: Option<String>,
    },
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
            AppError::InternalServerError(msg) => APIError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_message: msg,
                user_message: "Something went wrong. Please try again after sometime!".into(),
            },
            AppError::Message {
                status_code,
                error_message,
                user_message,
            } => {
                let user_msg = user_message.unwrap_or_else(|| error_message.clone());
                APIError {
                    status_code,
                    error_message,
                    user_message: user_msg,
                }
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        APIError::from(self).into_response()
    }
}
