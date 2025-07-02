use dotenv::dotenv;
use rust_axum_boilerplate::db::migration::Migrator;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None).await.expect("Migration failed");
}
