use axum::{middleware::from_fn, routing::{get, post}, Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{handlers, utils};

pub fn guarded_routes() -> Router {
    Router::new()
        .route("/protected", get(handlers::auth_handler::protected))
        .layer(from_fn(utils::guards::auth_guard))
}

pub fn unguarded_routes() -> Router {
    Router::new()
        .route("/health", get(handlers::health_handler::health))
        .route("/login", post(handlers::auth_handler::user_login))
        .route("/register", post(handlers::auth_handler::user_register))
}

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .merge(unguarded_routes())
        .merge(guarded_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
}
