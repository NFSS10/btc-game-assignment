use anyhow::Result;
use uuid::Uuid;

use crate::domain::player::PlayerState;
use crate::repositories::player_repository::PlayerRepository;

#[derive(Clone)]
pub struct PlayerService {
    repository: PlayerRepository,
}

impl PlayerService {
    pub fn new(repository: &PlayerRepository) -> Self {
        Self {
            repository: repository.clone(),
        }
    }

    pub async fn get_state_or_create(&self, player_id: Option<Uuid>) -> Result<PlayerState> {
        // if ID is provided, try to find the player in the database
        if let Some(id) = player_id {
            // if player exists, return it
            if let Some(existing) = self.repository.find_by_id(id).await? {
                return Ok(existing);
            }
        }

        // otherwise, create a new player and return it
        let created: PlayerState = self.repository.create().await?;
        Ok(created)
    }
}
