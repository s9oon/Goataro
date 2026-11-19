mod health;

use axum::{Router, routing::get};

use crate::state::AppState;

/// Every route in the app. Add new ones here.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .with_state(state)
}
