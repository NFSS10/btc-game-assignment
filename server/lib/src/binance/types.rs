use serde::Deserialize;
use serde_this_or_that::{as_f64, as_u64};

#[derive(Debug, Clone, Deserialize)]
pub struct TradeMessage {
    /// Server timestamp (ms) when Binance emitted the event
    #[serde(rename = "E")]
    #[serde(deserialize_with = "as_u64")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    #[serde(deserialize_with = "as_u64")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    #[serde(deserialize_with = "as_f64")]
    pub price: f64,

    #[serde(rename = "q")]
    #[serde(deserialize_with = "as_f64")]
    pub quantity: f64,

    /// Timestamp (ms) when the trade occurred
    #[serde(rename = "T")]
    #[serde(deserialize_with = "as_u64")]
    pub trade_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
}
