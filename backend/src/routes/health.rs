use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

/// 200 if the database answers, 500 if it doesn't.
pub async fn health(state: State<AppState>) -> Result<(), StatusCode> {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("health check failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
