use std::sync::Arc;

use anyhow::Result;
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use sea_orm::PaginatorTrait;
use uuid::Uuid;

use crate::db::config::PRICE_SCALE;
use crate::db::schemas::guesses;
use crate::domain::guess::{GuessDirection, SubmittedGuess};
use lib::utils::number_scaler::NumberScaler;

type SharedNumberScaler = Arc<NumberScaler>;

// TODO: register_guess is prone to race condition (not critical for the use case)

#[derive(Clone)]
pub struct GuessRepository {
    db: DatabaseConnection,
    price_scaler: SharedNumberScaler,
}
impl GuessRepository {
    pub fn new(db: &DatabaseConnection) -> Result<Self> {
        let price_scaler = NumberScaler::try_new(PRICE_SCALE)?;

        Ok(Self {
            db: db.clone(),
            price_scaler: Arc::new(price_scaler),
        })
    }

    pub async fn has_unresolved_guess(&self, player_id: Uuid) -> Result<bool> {
        let count = guesses::Entity::find()
            .filter(guesses::Column::PlayerId.eq(player_id))
            .filter(guesses::Column::ResolvedAt.is_null())
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    pub async fn register_guess(
        &self,
        player_id: Uuid,
        direction: GuessDirection,
        price: f64,
    ) -> Result<SubmittedGuess> {
        let entry_price_scaled = self.price_scaler.to_scaled_number(price)?;
        let db_direction = match direction {
            GuessDirection::Up => guesses::GuessDirection::Up,
            GuessDirection::Down => guesses::GuessDirection::Down,
        };

        let created = guesses::ActiveModel {
            player_id: Set(player_id),
            direction: Set(db_direction),
            entry_price_scaled: Set(entry_price_scaled),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(SubmittedGuess {
            id: created.id,
            created_at: created.created_at.timestamp_millis() as u64,
            entry_price: price,
            direction: direction,
        })
    }
}
