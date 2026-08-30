use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // enforce at most one unresolved guess per player
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE UNIQUE INDEX IF NOT EXISTS idx_guesses_one_unresolved_per_player
                ON guesses (player_id)
                WHERE resolved_at IS NULL
                "#
                .to_owned(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"DROP INDEX IF EXISTS idx_guesses_one_unresolved_per_player"#.to_owned(),
            ))
            .await?;

        Ok(())
    }
}