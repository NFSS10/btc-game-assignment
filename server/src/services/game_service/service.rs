use std::sync::Arc;
use tokio::sync::{RwLock, watch};

use super::price_feed::spawn_price_feed_ws;
use super::types::{LatestTick, WsPriceEvent};

type SharedLatestTick = Arc<RwLock<LatestTick>>;

#[derive(Clone)]
pub struct GameService {
    ws_shutdown_tx: watch::Sender<bool>,
    ws_event_receiver: kanal::AsyncReceiver<WsPriceEvent>,

    latest_tick: SharedLatestTick,
}
impl GameService {
    pub fn new(symbol: &str) -> Self {
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
            ws_shutdown_tx: ws_shutdown_tx,
            ws_event_receiver: ws_event_receiver,
            latest_tick: latest_tick,
        }
    }

    pub async fn run(&self) {
        // start a background task that acts on the price feed websocket events
        let ws_event_receiver = self.ws_event_receiver.clone();
        let context = TickContext {
            latest_tick: self.latest_tick.clone(),
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
}
async fn on_price_tick(price: f64, timestamp: u64, context: &TickContext) {
    // update the latest tick in the shared state
    {
        let mut latest_tick = context.latest_tick.write().await;
        latest_tick.price = price;
        latest_tick.timestamp = timestamp;
    }

    println!("Price tick: {} at {}", price, timestamp);
}
