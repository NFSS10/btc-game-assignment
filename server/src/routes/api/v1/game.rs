use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::Stream;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::AppState;
use crate::domain::guess::{GuessDirection, SubmittedGuess};
use crate::services::game_service::types::GameEvent;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/guess", post(guess))
        .route("/events", get(game_events))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuessBody {
    player_id: Uuid,
    direction: GuessDirection,
}

#[derive(Debug, Serialize)]
struct GuessResponse {
    pub accepted: bool,
    pub guess: Option<SubmittedGuess>,
}

async fn guess(
    State(state): State<AppState>,
    Json(body): Json<GuessBody>,
) -> Result<Json<GuessResponse>, StatusCode> {
    let guess_opt: Option<SubmittedGuess> = state
        .game_service
        .submit_guess(body.player_id, body.direction)
        .await
        .map_err(|err| {
            eprintln!("submit guess error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(GuessResponse {
        accepted: guess_opt.is_some(),
        guess: guess_opt,
    }))
}

#[derive(Debug, Deserialize)]
struct EventsQuery {
    #[serde(rename = "playerId")]
    player_id: Option<Uuid>,
}

async fn game_events(
    State(state): State<AppState>,
    Query(query): Query<EventsQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // extract the player_id from the query parameters
    let player_id = query.player_id;

    // create a shutdown signal future that resolves when the global shutdown signal is received
    let mut shutdown_rx = state.global_shutdown_rx.clone();
    let shutdown_signal = async move {
        if !*shutdown_rx.borrow() {
            let _ = shutdown_rx.changed().await;
        }
    };

    // subscribe to the game events broadcast channel
    let events_receiver = state.game_service.subscribe_events();

    // create a stream that converts the broadcast events into SSE events
    // and stop when the shutdown signal is received
    let stream = BroadcastStream::new(events_receiver)
        .filter_map(|result| async move { result.ok() })
        .filter(move |event| {
            let should_send = match event {
                // everyone receives global price changes
                GameEvent::PriceChange { .. } => true,

                // Only send to the matching player (if player_id was supplied)
                GameEvent::GuessResolved { guess_state } => {
                    Some(guess_state.player_id) == player_id
                }
                GameEvent::ScoreUpdate {
                    player_id: event_player_id,
                    ..
                } => Some(*event_player_id) == player_id,
            };
            async move { should_send }
        })
        .map(|event| match event {
            GameEvent::PriceChange { price, timestamp } => {
                let payload = json!({
                    "price": price,
                    "timestamp": timestamp,
                });
                Ok(json_event("price_change", payload))
            }
            GameEvent::GuessResolved { guess_state } => {
                let payload = json!({
                    "guessId": guess_state.guess_id,
                    "playerId": guess_state.player_id,
                    "entryPrice": guess_state.entry_price,
                    "direction": match guess_state.direction {
                        GuessDirection::Up => "up",
                        GuessDirection::Down => "down",
                    },
                    "createdAt": guess_state.created_at,
                    "resolvedPrice": guess_state.resolved_price,
                    "resolvedAt": guess_state.resolved_at,
                    "isCorrect": guess_state.is_correct,
                });
                Ok(json_event("guess_resolved", payload))
            }
            GameEvent::ScoreUpdate {
                player_id,
                new_score,
            } => {
                let payload = json!({
                    "playerId": player_id,
                    "newScore": new_score,
                });
                Ok(json_event("score_update", payload))
            }
        })
        .take_until(shutdown_signal);

    // wrap the stream in an SSE response with a keep-alive ping every 15 seconds
    return Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    );
}

fn json_event(event: &str, json_value: Value) -> Event {
    Event::default().event(event).data(json_value.to_string())
}
