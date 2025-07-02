// Authentication related handlers

use crate::types::custom_json_decoder_types::CustomJson;
use crate::{
    db::entity::users,
    types::{
        auth_types::{LoginReq, LoginRes, RegisterReq},
        error_types::AppError,
        util_types::GenericJsonRes,
    },
    utils::jwt,
};
use axum::{Extension, Json, http::header, response::IntoResponse};
use bcrypt::{hash, verify};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

pub async fn user_register(
    Extension(db): Extension<DatabaseConnection>,
    CustomJson(payload): CustomJson<RegisterReq>,
) -> Result<GenericJsonRes, AppError> {
    let hashed =
        hash(payload.password, 4).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(payload.username),
        password_hash: Set(hashed),
    };

    match users::Entity::insert(user).exec(&db).await {
        Ok(_) => Ok(GenericJsonRes {
            data: json!({ "status": "ok" }),
        }),
        Err(e) => Err(AppError::InternalServerError(e.to_string())),
    }
}

pub async fn user_login(
    Extension(db): Extension<DatabaseConnection>,
    CustomJson(payload): CustomJson<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(payload.username))
        .one(&db)
        .await
        .unwrap();

    if let Some(u) = user {
        if verify(payload.password, &u.password_hash).unwrap_or(false) {
            if let Ok(token) = jwt::encode_jwt(u.id) {
                let body = Json(json!(LoginRes {
                    token: token.clone()
                }));
                let mut response = body.into_response();
                let cookie_value = format!(
                    "auth_token={}; HttpOnly; Secure; SameSite=Lax; Path=/",
                    token
                );
                response
                    .headers_mut()
                    .insert(header::SET_COOKIE, cookie_value.parse().unwrap());
                return Ok(response);
            }
        }
    }

    Err(AppError::Unauthorized)
}

pub async fn protected() -> impl IntoResponse {
    Json(json!({ "message": "protected" }))
}
