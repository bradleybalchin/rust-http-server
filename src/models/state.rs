use sqlx::PgPool;

// Database state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}