use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct GenericJsonReq {
    pub value: Value,
}

#[derive(Debug)]
pub struct GenericJsonRes {
    pub data: Value,
}

impl IntoResponse for GenericJsonRes {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self.data)).into_response()
    }
}
