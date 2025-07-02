use axum::{
    extract::Request,
    http::header::{AUTHORIZATION, COOKIE},
    middleware::Next,
    response::{IntoResponse, Response},
};

use uuid::Uuid;

use crate::types::error_types::AppError;
use crate::utils::{constants::TOKEN_PREFIX, jwt};

pub async fn jwt_guard(mut req: Request, next: Next) -> Response {
    if let Some(value) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = value.to_str() {
            let prefix = format!("{} ", TOKEN_PREFIX);
            if let Some(token) = auth_str.strip_prefix(&prefix) {
                if let Ok(user_id) = jwt::decode_jwt(token) {
                    req.extensions_mut().insert::<Uuid>(user_id);
                    return next.run(req).await;
                }
            }
        }
    }

    if let Some(cookie_header) = req.headers().get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for part in cookie_str.split(';') {
                let part = part.trim();
                if let Some(token) = part.strip_prefix("auth_token=") {
                    if let Ok(user_id) = jwt::decode_jwt(token) {
                        req.extensions_mut().insert::<Uuid>(user_id);
                        return next.run(req).await;
                    }
                }
            }
        }
    }
    AppError::Unauthorized.into_response()
}
