use axum::{
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};

use uuid::Uuid;

use crate::utils::{constants::TOKEN_PREFIX, jwt};

pub async fn jwt_guard<B>(mut req: Request<B>, next: Next<B>) -> Response {
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
    StatusCode::UNAUTHORIZED.into_response()
}
