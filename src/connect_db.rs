use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

pub async fn connect_db() -> PgPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database")

}