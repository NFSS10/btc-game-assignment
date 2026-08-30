use migration::sea_orm::prelude::Uuid;
use serde::Serialize;

use crate::db::schemas::players;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub id: Uuid,
    pub score: i32,
}

impl From<players::Model> for PlayerState {
    fn from(value: players::Model) -> Self {
        Self {
            id: value.id,
            score: value.score,
        }
    }
}
