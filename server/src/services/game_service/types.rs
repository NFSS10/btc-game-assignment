use uuid::Uuid;

use crate::domain::guess::ResolvedGuess;

/// Represents a game event that can be sent from the game service to its subscribers
#[derive(Debug, Clone)]
pub enum GameEvent {
    GuessResolved { guess_state: ResolvedGuess },
    ScoreUpdate { player_id: Uuid, new_score: i32 },
    PriceChange { price: f64, timestamp: u64 },
}

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
