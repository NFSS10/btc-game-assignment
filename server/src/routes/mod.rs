use axum::Router;

use crate::AppState;

mod health;

pub fn router() -> Router<AppState> {
    Router::new().nest("/health", health::router())
}
