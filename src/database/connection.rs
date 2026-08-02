use sqlx::{pool, postgres::{PgPool, PgPoolOptions}};
use dotenvy::dotenv;
use std::env;

// Connect to database
pub async fn connect() -> Result<PgPool,sqlx::Error> {
    
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await

}