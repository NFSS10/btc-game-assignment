/// Represents a price event received from the websocket feed
#[derive(Debug, Clone)]
pub enum WsPriceEvent {
    Trade { price: f64, timestamp: u64 },
}

/// Represents the latest tick received from the price feed
#[derive(Debug, Clone)]
pub struct LatestTick {
    pub price: f64,
    pub timestamp: u64,
}
