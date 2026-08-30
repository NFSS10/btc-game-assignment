use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuessDirection {
    #[serde(rename = "up", alias = "Up", alias = "UP")]
    Up,
    #[serde(rename = "down", alias = "Down", alias = "DOWN")]
    Down,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmittedGuess {
    pub id: i32,
    pub created_at: u64,
    pub entry_price: f64,
    pub direction: GuessDirection,
}
