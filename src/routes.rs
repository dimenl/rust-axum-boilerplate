use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{handlers, utils};

pub fn guarded_routes() -> Router {
    Router::new()
        .route("/api/protected", get(handlers::auth_handler::protected))
        .layer(from_fn(utils::guards::jwt_guard))
}

pub fn unguarded_routes() -> Router {
    Router::new()
        .route("/api/health", get(handlers::health_handler::health))
        .route("/api/auth/login", post(handlers::auth_handler::user_login))
        .route(
            "/api/auth/logout",
            post(handlers::auth_handler::user_logout),
        )
        .route(
            "/api/auth/register",
            post(handlers::auth_handler::user_register),
        )
}

pub fn create_router() -> Router {
    Router::new()
        .merge(unguarded_routes())
        .merge(guarded_routes())
}
