use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast, watch};
use uuid::Uuid;

use super::price_feed::spawn_price_feed_ws;
use super::types::{GameEvent, LatestTick, WsPriceEvent};
use crate::domain::guess::{GuessDirection, SubmittedGuess};
use crate::repositories::guess_repository::GuessRepository;

type SharedLatestTick = Arc<RwLock<LatestTick>>;

// TODO:
// - need to handle the case where for some reason the server starts and there is guesses
//   that are unresolved. We need to resolve them and determine if they were correct or not

#[derive(Clone)]
pub struct GameService {
    guess_repository: GuessRepository,

    ws_shutdown_tx: watch::Sender<bool>,
    ws_event_receiver: kanal::AsyncReceiver<WsPriceEvent>,

    latest_tick: SharedLatestTick,
    event_sender: broadcast::Sender<GameEvent>,
}
impl GameService {
    pub fn new(symbol: &str, guess_repository: &GuessRepository) -> Self {
        // create a broadcast channel for the game events
        let (event_sender, _) = broadcast::channel(1024);

        // create channel that will handle WS events from the price feed websocket
        let (ws_event_sender, ws_event_receiver) = kanal::unbounded_async::<WsPriceEvent>();

        // spawn the price feed websocket task to get the latest price updates
        let symbol_lower = symbol.to_lowercase();
        let ws_shutdown_tx = spawn_price_feed_ws(&symbol_lower, &ws_event_sender);

        let latest_tick = Arc::new(RwLock::new(LatestTick {
            price: -999_999.0,
            timestamp: 0,
        }));

        Self {
            guess_repository: guess_repository.clone(),
            ws_shutdown_tx: ws_shutdown_tx,
            ws_event_receiver: ws_event_receiver,
            latest_tick: latest_tick,
            event_sender: event_sender,
        }
    }

    pub async fn run(&self) {
        // start a background task that acts on the price feed websocket events
        let ws_event_receiver = self.ws_event_receiver.clone();
        let context = TickContext {
            latest_tick: self.latest_tick.clone(),
            event_sender: self.event_sender.clone(),
        };
        tokio::spawn(async move {
            while let Ok(event) = ws_event_receiver.recv().await {
                match event {
                    WsPriceEvent::Trade { price, timestamp } => {
                        on_price_tick(price, timestamp, &context).await
                    }
                }
            }
        });

        // wait until there is one tick update before continuing
        loop {
            let latest_tick = self.get_latest_tick().await;
            if latest_tick.price > 0.0 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Submit a guess for a player. Returns true if the guess was accepted, false if the player is in cooldown.
    pub async fn submit_guess(
        &self,
        player_id: Uuid,
        direction: GuessDirection,
    ) -> Result<Option<SubmittedGuess>> {
        let has_unresolved_guess = self
            .guess_repository
            .has_unresolved_guess(player_id)
            .await?;
        if has_unresolved_guess {
            return Ok(None);
        }

        let latest_tick = self.get_latest_tick().await;

        let submitted: SubmittedGuess = self
            .guess_repository
            .register_guess(player_id, direction, latest_tick.price)
            .await?;

        Ok(Some(submitted))
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<GameEvent> {
        let receiver = self.event_sender.subscribe();
        return receiver;
    }

    pub fn destroy(&self) {
        let _ = self.ws_shutdown_tx.send(true);
    }

    /// safely get the latest tick from the shared state
    async fn get_latest_tick(&self) -> LatestTick {
        let value_guard = self.latest_tick.read().await;
        return value_guard.clone();
    }
}

struct TickContext {
    latest_tick: SharedLatestTick,
    event_sender: broadcast::Sender<GameEvent>,
}
async fn on_price_tick(price: f64, timestamp: u64, context: &TickContext) {
    let has_price_changed: bool;
    // update the latest tick in the shared state
    {
        let mut latest_tick = context.latest_tick.write().await;
        has_price_changed = price != latest_tick.price;

        latest_tick.price = price;
        latest_tick.timestamp = timestamp;
    }

    if !has_price_changed {
        return;
    }

    // TODO: update the game state here???
    println!("Price tick: {} at {}", price, timestamp);

    // broadcast the price change event to all subscribers
    let _ = context
        .event_sender
        .send(GameEvent::PriceChange { price, timestamp });
}
