use sqlx::postgres::{PgPool, PgPoolOptions};

/// Shared dependencies, handed to every handler through `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}
