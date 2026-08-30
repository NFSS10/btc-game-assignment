use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use migration::sea_orm::prelude::Uuid;
use serde::Deserialize;

use crate::AppState;
use crate::services::player_service::types::PlayerState;

pub fn router() -> Router<AppState> {
    Router::new().route("/init", post(init))
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
