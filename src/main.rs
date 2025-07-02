mod handlers;
mod routes;
mod utils;
mod types;
mod db;

use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use utils::DATABASE_URL;

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect(DATABASE_URL.as_str())
        .await
        .expect("Failed to connect to database");

    let app: Router = routes::create_router(db.clone());

    let addr = "0.0.0.0:5000";
    println!("Server running on {addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
