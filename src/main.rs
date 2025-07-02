mod handlers;
mod routes;
mod utils;
mod types;
mod db;

use axum::Router;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let app: Router = routes::create_routes(db.clone());

    let addr = "0.0.0.0:5000";
    println!("Server running on {addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
