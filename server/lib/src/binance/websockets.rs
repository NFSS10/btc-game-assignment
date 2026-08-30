// Reference: https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/~#trade

use std::future::Future;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

use crate::utils::websocket_client::WebSocketClient;

use super::types::TradeMessage;

pub enum TradeWebsocketEvent {
    Trade(TradeMessage),
}

pub struct TradeWebsocket {
    client: WebSocketClient,
}
impl TradeWebsocket {
    pub async fn connect<F, Fut>(symbol: &str, on_event: F) -> Self
    where
        F: Fn(TradeWebsocketEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let endpoint = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol);

        // wrap the callback in Arc so it can be moved into the async closure
        let on_event = Arc::new(on_event);

        // define message handler
        let on_message = move |msg: Message| {
            let on_event = Arc::clone(&on_event);
            async move {
                on_message_call(msg, on_event).await;
            }
        };

        // define on_connect handler to send subscription on each (re)connection
        let on_connect = move |_tx: mpsc::Sender<Message>| async move {};

        // connect to websocket
        let client = WebSocketClient::connect(&endpoint, on_connect, on_message, None).await;

        Self { client: client }
    }

    pub async fn close(&mut self) {
        self.client.close().await;
    }
}

async fn on_message_call<F, Fut>(msg: Message, on_event: Arc<F>)
where
    F: Fn(TradeWebsocketEvent) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    if let Message::Text(text) = msg {
        match serde_json::from_str::<TradeMessage>(&text) {
            Ok(trade) => {
                (on_event)(TradeWebsocketEvent::Trade(trade)).await;
            }
            Err(e) => {
                eprintln!("🔴 Failed to parse TradeMessage: {e:?} | msg: {text}");
            }
        }
    }
}
