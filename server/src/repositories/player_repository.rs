use anyhow::Result;
use migration::sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::db::schemas::players;
use crate::domain::player::PlayerState;

#[derive(Clone)]
pub struct PlayerRepository {
    db: DatabaseConnection,
}
impl PlayerRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlayerState>> {
        let player = players::Entity::find_by_id(id).one(&self.db).await?;

        Ok(player.map(Into::into))
    }

    pub async fn create(&self) -> Result<PlayerState> {
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
