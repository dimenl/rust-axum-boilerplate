// Health check handler

use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}
