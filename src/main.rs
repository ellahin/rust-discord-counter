use dotenvy::dotenv;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::SqlitePool;
use std::env;
use std::path::Path;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().expect(".env file not found");

    if env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL not in environment vars");
    }

    let database_url = env::var("DATABASE_URL").unwrap();

    if !sqlx::Sqlite::database_exists(&database_url).await.unwrap() {
        sqlx::Sqlite::create_database(&database_url).await.unwrap();
    }

    let migration_path = Path::new("./migrations");

    let sql_pool = SqlitePool::connect(&database_url).await.unwrap();

    Migrator::new(migration_path)
        .await
        .unwrap()
        .run(&sql_pool)
        .await
        .unwrap();
}
