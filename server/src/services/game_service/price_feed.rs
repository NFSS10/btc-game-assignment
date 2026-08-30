use lib::binance::websockets::{TradeWebsocket, TradeWebsocketEvent};
use tokio::sync::watch;

use super::types::WsPriceEvent;

/// Starts a background websocket task that listens to Binance trade events.
/// It will send `WsPriceEvent` messages to the provided `event_sender` channel whenever a trade event is received.
///
/// # Returns
/// A `watch::Sender<bool>` that can be used to signal the websocket task to shut down.
pub fn spawn_price_feed_ws(
    symbol: &str,
    event_sender: &kanal::AsyncSender<WsPriceEvent>,
) -> watch::Sender<bool> {
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    // own variables to move into the async task
    let symbol = symbol.to_owned();
    let ctx = CallbackContext {
        event_sender: event_sender.clone(),
    };

    // create a background task that runs the websocket connection
    tokio::spawn(async move {
        let on_trade_event_cb = move |event: TradeWebsocketEvent| {
            let ctx = ctx.clone();
            async move {
                on_trade_event(&event, &ctx).await;
            }
        };
        let mut ws = TradeWebsocket::connect(&symbol, on_trade_event_cb).await;

        // keep the websocket running until shutdown signal is received
        while !*shutdown_rx.borrow() {
            if shutdown_rx.changed().await.is_err() {
                break;
            }
        }

        ws.close().await;
    });

    return shutdown_tx;
}

#[derive(Clone)]
struct CallbackContext {
    event_sender: kanal::AsyncSender<WsPriceEvent>,
}
async fn on_trade_event(event: &TradeWebsocketEvent, context: &CallbackContext) {
    match event {
        TradeWebsocketEvent::Trade(msg) => {
            let ws_event = WsPriceEvent::Trade {
                price: msg.price,
                timestamp: msg.trade_time,
            };

            let result = context.event_sender.send(ws_event).await;
            if let Err(e) = result {
                eprintln!("Failed to send trade event to channel: {}", e);
            }
        }
    }
}
