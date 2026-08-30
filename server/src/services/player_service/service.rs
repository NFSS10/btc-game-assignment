use anyhow::Result;
use migration::sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

use super::types::PlayerState;
use crate::db::schemas::players;

// TODO: create player repository?

#[derive(Clone)]
pub struct PlayerService {
    db: DatabaseConnection,
}

impl PlayerService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get_state_or_create(&self, player_id: Option<Uuid>) -> Result<PlayerState> {
        // if ID is provided, try to find the player in the database
        if let Some(id) = player_id {
            // if player exists, return it
            if let Some(existing) = players::Entity::find_by_id(id).one(&self.db).await? {
                return Ok(existing.into());
            }
        }

        // otherwise, create a new player and return it
        let created = players::ActiveModel {
            id: Set(Uuid::new_v4()),
            score: Set(0),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(created.into())
    }

    pub async fn apply_score_delta(&self, id: Uuid, delta: i32) -> Result<Option<PlayerState>> {
        // find the player by ID, if it exists otherwise return None
        let Some(model) = players::Entity::find_by_id(id).one(&self.db).await? else {
            return Ok(None);
        };

        // apply the delta to the player's score
        let next_score = model.score + delta;

        // update the player in the database and return the updated state
        let mut active = model.into_active_model();
        active.score = Set(next_score);
        let updated = active.update(&self.db).await?;

        let state = PlayerState::from(updated);

        Ok(Some(state))
    }
}
