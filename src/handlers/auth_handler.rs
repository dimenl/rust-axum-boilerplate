// Authentication related handlers

use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn login() -> impl IntoResponse {
    Json(json!({ "message": "login" }))
}

pub async fn protected() -> impl IntoResponse {
    Json(json!({ "message": "protected" }))
}
