use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health))
}

async fn health() -> Json<Value> {
    Json(json!({ "ok": true }))
}
