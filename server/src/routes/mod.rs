use axum::Router;

use crate::AppState;

mod api;
mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/health", health::router())
        .nest("/api", api::router())
}
