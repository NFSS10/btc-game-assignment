use axum::Router;

use crate::AppState;

mod game;

pub fn router() -> Router<AppState> {
    Router::new().nest("/game", game::router())
}
