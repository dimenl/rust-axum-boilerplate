// Authentication related handlers

use axum::{response::IntoResponse, Extension, Json};
use bcrypt::{hash, verify};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::{
    db::entity::users,
    types::auth_types::{AuthResponse, LoginRequest, RegisterRequest},
    utils::jwt,
};

pub async fn user_register(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let hashed = match hash(payload.password, 4) {
        Ok(h) => h,
        Err(_) => return Json(json!({ "error": "hash" })),
    };

    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(payload.username),
        password_hash: Set(hashed),
    };

    match users::Entity::insert(user).exec(&db).await {
        Ok(_) => Json(json!({ "status": "ok" })),
        Err(_) => Json(json!({ "error": "insert" })),
    }
}

pub async fn user_login(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(payload.username))
        .one(&db)
        .await
        .unwrap();

    if let Some(u) = user {
        if verify(payload.password, &u.password_hash).unwrap_or(false) {
            if let Ok(token) = jwt::encode_jwt(u.id) {
                return Json(json!(AuthResponse { token }));
            }
        }
    }

    Json(json!({ "error": "invalid credentials" }))
}

pub async fn protected() -> impl IntoResponse {
    Json(json!({ "message": "protected" }))
}
