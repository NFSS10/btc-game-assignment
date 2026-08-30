use axum::extract::State;
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
    println!("Received guess request: {:?}", body);

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

async fn game_events(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("Client connected to SSE events stream");

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
        .map(|event| match event {
            GameEvent::PriceChange { price, timestamp } => {
                let payload = json!({
                    "price": price,
                    "timestamp": timestamp,
                });

                Ok(json_event("price_change", payload))
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

#[derive(Debug, Clone)]
struct PriceChangeEvent {
    price: f64,
    timestamp: u64,
}
