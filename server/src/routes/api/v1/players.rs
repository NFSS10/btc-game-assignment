use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use migration::sea_orm::prelude::Uuid;
use serde::Deserialize;

use crate::AppState;
use crate::domain::guess::GuessDetails;
use crate::domain::player::PlayerState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/init", post(init))
        .route("/{player_id}/guesses", get(list_guesses))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitBody {
    player_id: Option<Uuid>,
}

async fn init(
    State(state): State<AppState>,
    Json(body): Json<InitBody>,
) -> Result<Json<PlayerState>, StatusCode> {
    let state: PlayerState = state
        .player_service
        .get_state_or_create(body.player_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(state))
}

async fn list_guesses(
    State(state): State<AppState>,
    Path(player_id): Path<Uuid>,
) -> Result<Json<Vec<GuessDetails>>, StatusCode> {
    let guesses: Vec<GuessDetails> = state
        .game_service
        .get_player_guesses(player_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(guesses))
}
