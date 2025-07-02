// Authentication related types

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRes {
    pub token: String,
}

impl IntoResponse for LoginRes {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(json!({ "token": self.token }))).into_response()
    }
}
