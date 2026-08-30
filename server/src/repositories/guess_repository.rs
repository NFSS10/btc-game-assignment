use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, TimeZone, Utc};
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use sea_orm::{IntoActiveModel, PaginatorTrait};
use uuid::Uuid;

use crate::db::config::PRICE_SCALE;
use crate::db::schemas::guesses;
use crate::domain::guess::{GuessDirection, ResolvedGuess, SubmittedGuess};
use lib::utils::number_scaler::NumberScaler;

type SharedNumberScaler = Arc<NumberScaler>;

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

        // insert the guess into the database
        // NOTE: Concurrency safety is enforced at DB level by a partial unique index:
        // one unresolved guess per player (`resolved_at IS NULL`).
        // See: m20260830_204715_only_one_unresolved_guess_per_player.rs
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

    pub async fn resolve_eligible_guesses(&self, current_price: f64, current_timestamp: u64) -> Result<Vec<ResolvedGuess>> {
        // scale the price to the same scale as the stored guesses
        let resolved_price_scaled = self.price_scaler.to_scaled_number(current_price)?;

        // convert the current timestamp (in milliseconds) to a chrono::DateTime<Utc>
        let current_timestamp_date = Utc
            .timestamp_millis_opt(current_timestamp as i64)
            .single()
            .ok_or_else(|| anyhow::anyhow!("invalid current_timestamp millis"))?;
    
        // calculate the cutoff timestamp for eligible guesses (older than 60 seconds)
        let cutoff = current_timestamp_date - Duration::seconds(60);

        // get the list of unresolved guesses that are older than the cutoff timestamp
        let candidates = guesses::Entity::find()
            .filter(guesses::Column::ResolvedAt.is_null())
            .filter(guesses::Column::CreatedAt.lte(cutoff))
            .all(&self.db)
            .await?;

        let mut resolved = Vec::new();
        for candidate in candidates {
            // edge case: if the resolved price is the same as the entry price, we consider it unresolved and skip it
            if candidate.entry_price_scaled == resolved_price_scaled {
                continue;
            }

            let is_correct = match candidate.direction {
                guesses::GuessDirection::Up => resolved_price_scaled > candidate.entry_price_scaled,
                guesses::GuessDirection::Down => resolved_price_scaled < candidate.entry_price_scaled,
            };

            // update the guess in the database to mark it as resolved
            let mut active = candidate.clone().into_active_model();
            active.resolved_at = Set(Some(current_timestamp_date.into()));
            active.resolved_price_scaled = Set(Some(resolved_price_scaled));
            active.update(&self.db).await?;

            let entry_price = self.price_scaler.from_scaled_number(candidate.entry_price_scaled)?;
            let entry_direction = match candidate.direction {
                guesses::GuessDirection::Up => GuessDirection::Up,
                guesses::GuessDirection::Down => GuessDirection::Down,
            };
            let resolved_guess = ResolvedGuess {
                guess_id: candidate.id,
                player_id: candidate.player_id,
                entry_price: entry_price,
                direction: entry_direction,
                created_at: candidate.created_at.timestamp_millis() as u64,
                resolved_price: current_price,
                resolved_at: current_timestamp,
                is_correct: is_correct,
            };

            resolved.push(resolved_guess);
        }

        Ok(resolved)
    }

}
