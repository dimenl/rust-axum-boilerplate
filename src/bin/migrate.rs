use rust_axum_boilerplate::db::migration::Migrator;
use sea_orm_migration::prelude::*;
use rust_axum_boilerplate::utils::DATABASE_URL;

#[tokio::main]
async fn main() {
    let db = sea_orm::Database::connect(DATABASE_URL.as_str())
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None).await.expect("Migration failed");
}
