use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::future::Future;
use std::sync::Arc;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::{Duration, Instant, sleep, timeout},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

/// A WebSocket client that automatically reconnects on disconnect and supports ping/pong.
pub struct WebSocketClient {
    shutdown_tx: Option<oneshot::Sender<()>>,
    send_tx: mpsc::Sender<Message>,
    handle: JoinHandle<()>,
}
impl WebSocketClient {
    pub async fn connect<F, Fut, C, CFut>(
        uri: &str,
        on_connect: C,
        on_message: F,
        ping_interval: Option<Duration>,
    ) -> Self
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        C: Fn(mpsc::Sender<Message>) -> CFut + Send + Sync + 'static,
        CFut: Future<Output = ()> + Send + 'static,
    {
        // channel to signal shutdown
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        // channel to allow sending messages to the websocket task
        let (send_tx, mut send_rx) = mpsc::channel::<Message>(32);

        // sender clone for ping task
        let ping_tx = send_tx.clone();

        // sender clone for on_connect callback
        let connect_tx = send_tx.clone();

        // wrap on_connect in Arc so it can be called on each reconnection
        let on_connect = Arc::new(on_connect);

        let uri = uri.to_string();
        let handle = tokio::spawn(async move {
            // outer loop that forces reconnection when the websocket's main loop exits
            loop {
                // start connection (with timeout)
                let ws_stream = match timeout(Duration::from_secs(10), connect_async(&uri)).await {
                    Ok(Ok((ws, _))) => ws,
                    Ok(Err(e)) => {
                        eprintln!("Failed to connect: {e}, retrying in 2.25s...");
                        sleep(Duration::from_millis(2250)).await;
                        continue;
                    }
                    Err(_) => {
                        eprintln!("Connection timeout, retrying in 2.25s...");
                        sleep(Duration::from_millis(2250)).await;
                        continue;
                    }
                };
                let (mut write, mut read) = ws_stream.split();

                // call on_connect callback (e.g., to send subscriptions)
                on_connect(connect_tx.clone()).await;

                // cancellation token for ping task (cancelled on disconnect)
                let ping_cancel = CancellationToken::new();

                // pong timeout tracking
                let pong_timeout = Duration::from_secs(7);
                let mut last_pong = Instant::now();

                // setup ping task (if enabled)
                if let Some(interval) = ping_interval {
                    let ping_tx = ping_tx.clone();
                    let ping_cancel = ping_cancel.clone();
                    tokio::spawn(async move {
                        loop {
                            tokio::select! {
                                _ = sleep(interval) => {
                                    if ping_tx.send(Message::Ping(Vec::new().into())).await.is_err() {
                                        break;
                                    }
                                }
                                _ = ping_cancel.cancelled() => break,
                            }
                        }
                    });
                }

                // main loop that handles the websocket messages
                loop {
                    // calculate how much time is left until the pong timeout
                    let pong_timeout_deadline = if ping_interval.is_some() {
                        (last_pong + ping_interval.unwrap() + pong_timeout)
                            .saturating_duration_since(Instant::now())
                    } else {
                        Duration::from_secs(86400) // effectively infinite, should never trigger.
                    };

                    tokio::select! {
                        // pong timeout
                        // (basically if this select branch is hit, it means we haven't received a pong in time)
                        _ = sleep(pong_timeout_deadline), if ping_interval.is_some() => {
                            eprintln!("Pong timeout, reconnecting...");
                            break;
                        }
                        // read messages
                        maybe_msg = read.next() => {
                            match maybe_msg {
                                Some(Ok(Message::Pong(_))) => {
                                    last_pong = Instant::now();
                                }
                                Some(Ok(Message::Ping(payload))) => {
                                    // respond to ping with pong (echo payload)
                                    if let Err(e) = write.send(Message::Pong(payload)).await {
                                        eprintln!("WebSocket pong send error: {e}");
                                        break;
                                    }
                                },
                                Some(Ok(Message::Close(frame))) => {
                                    eprintln!("WebSocket close frame received; closing this session.");
                                    let _ = write.send(Message::Close(frame)).await; // best-effort close handshake
                                    break; // outer loop decides reconnect policy
                                }
                                Some(Ok(msg)) => on_message(msg).await,
                                Some(Err(e)) => {
                                    eprintln!("WebSocket error: {e}");
                                    break;
                                }
                                None => {
                                    // stream ended
                                    break;
                                }
                            }
                        },
                        // write messages
                        Some(msg) = send_rx.recv() => {
                            if let Err(e) = write.send(msg).await {
                                eprintln!("WebSocket send error: {e}");
                                break;
                            }
                        },
                        // shutdown
                        _ = &mut shutdown_rx => {
                            println!("WebSocket closing...");
                            let _ = write.close().await;
                            ping_cancel.cancel();
                            return;
                        }
                    }
                }

                // cancel ping task before reconnecting
                ping_cancel.cancel();

                eprintln!("WebSocket disconnected, reconnecting in 1.5s...");
                sleep(Duration::from_millis(1500)).await;
            }
        });

        Self {
            shutdown_tx: Some(shutdown_tx),
            send_tx: send_tx,
            handle: handle,
        }
    }

    /// Send a message to the websocket.
    pub async fn send(&self, msg: Message) -> Result<()> {
        self.send_tx.send(msg).await?;
        Ok(())
    }

    /// Manually close the websocket and stop reconnecting.
    pub async fn close(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        // replace the handle with a dummy and await the old one
        let handle = std::mem::replace(&mut self.handle, tokio::spawn(async {}));
        let _ = handle.await;
    }
}
