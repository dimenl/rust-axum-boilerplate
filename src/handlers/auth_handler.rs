// Authentication related handlers

use axum::{Extension, Json, response::IntoResponse};
use bcrypt::{hash, verify};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::{
    db::entity::users,
    types::{
        auth_types::{AuthResponse, LoginRequest, RegisterRequest},
        error_types::AppError,
    },
    utils::jwt,
};

pub async fn user_register(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let hashed = hash(payload.password, 4).map_err(|_| AppError::Message("hash".into()))?;

    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(payload.username),
        password_hash: Set(hashed),
    };

    match users::Entity::insert(user).exec(&db).await {
        Ok(_) => Ok(Json(json!({ "status": "ok" }))),
        Err(_) => Err(AppError::Message("insert".into())),
    }
}

pub async fn user_login(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(payload.username))
        .one(&db)
        .await
        .unwrap();

    if let Some(u) = user {
        if verify(payload.password, &u.password_hash).unwrap_or(false) {
            if let Ok(token) = jwt::encode_jwt(u.id) {
                return Ok(Json(json!(AuthResponse { token })));
            }
        }
    }

    Err(AppError::Unauthorized)
}

pub async fn protected() -> impl IntoResponse {
    Json(json!({ "message": "protected" }))
}
