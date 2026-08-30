use axum::Router;

use crate::AppState;

mod game;
mod players;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/game", game::router())
        .nest("/players", players::router())
}
