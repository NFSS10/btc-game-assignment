use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::get;
use axum::Router;
use serde_json::{Value, json};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};

use crate::AppState;
use crate::services::game_service::types::GameEvent;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/events", get(game_events))
}

async fn game_events(State(state): State<AppState>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("Client connected to SSE events stream");

    // subscribe to the game events broadcast channel
    let events_receiver = state.game_service.subscribe_events();

    // create a stream that converts the broadcast events into SSE events
    let stream = BroadcastStream::new(events_receiver)
        .filter_map(|result| result.ok())
        .map(move |event| {
            match event {
                GameEvent::PriceChange { price, timestamp } => {
                    let price_change_event = PriceChangeEvent { price, timestamp };
                    let payload = json!({
                        "price": price_change_event.price,
                        "timestamp": price_change_event.timestamp,
                    });

                    return Ok(json_event("price_change", payload));
                }
            }
        });

    // wrap the stream in an SSE response with a keep-alive ping every 15 seconds
    return Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    );
}

fn json_event(event: &str, json_value: Value) -> Event {
    Event::default()
        .event(event)
        .data(json_value.to_string())
}

#[derive(Debug, Clone)]
struct PriceChangeEvent {
    price: f64,
    timestamp: u64,
}

