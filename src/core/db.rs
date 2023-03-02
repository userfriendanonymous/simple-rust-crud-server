use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create(url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
    .max_connections(4)
    .connect(url)
    .await
    .expect("Error creating db pool")
}