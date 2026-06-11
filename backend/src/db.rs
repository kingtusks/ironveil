use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn create_pool() -> AppState {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("POSTGRES_URL").expect("postgres_url not set in .env"))
        .await
        .expect("failed to connect to postgres");

    AppState {pool}
}