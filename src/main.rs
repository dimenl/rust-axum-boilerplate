mod db;
mod handlers;
mod routes;
mod types;
mod utils;

use axum::middleware::from_fn;
use axum::{Extension, extract::DefaultBodyLimit, http::HeaderName, response::IntoResponse};
use sea_orm::{Database, DatabaseConnection};
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::{DATABASE_URL, LOG_DIR, logging};

async fn not_found() -> impl IntoResponse {
    types::error_types::AppError::NotFound
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily(LOG_DIR.as_str(), "app.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(file_writer),
        )
        .init();

    let db: DatabaseConnection = Database::connect(DATABASE_URL.as_str())
        .await
        .expect("Failed to connect to database");

    let app = routes::create_router()
        .fallback(not_found)
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .layer(Extension(db))
        .layer(from_fn(logging::logger))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid::default(),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = "0.0.0.0:5000";
    tracing::info!("Server running on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
