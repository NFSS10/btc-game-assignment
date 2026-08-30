use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guesses::Table)
                    .if_not_exists()
                    // id: i32
                    .col(pk_auto(Guesses::Id))
                    // created_at
                    .col(timestamp_with_time_zone(Guesses::CreatedAt).not_null())
                    // updated_at
                    .col(timestamp_with_time_zone(Guesses::UpdatedAt).not_null())
                    // resolved_at: Option<DateTimeWithTimeZone>
                    .col(
                        ColumnDef::new(Guesses::ResolvedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    // player_id: Uuid (foreign key to players table)
                    .col(ColumnDef::new(Guesses::PlayerId).uuid().not_null())
                    // direction: GuessDirection enum stored as string
                    .col(string(Guesses::Direction).not_null())
                    // entry_price_scaled: i64
                    .col(big_integer(Guesses::EntryPriceScaled).not_null())
                    // resolved_price_scaled: Option<i64>
                    .col(
                        ColumnDef::new(Guesses::ResolvedPriceScaled)
                            .big_integer()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guesses_player_id")
                            .from(Guesses::Table, Guesses::PlayerId)
                            .to(Players::Table, Players::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_guesses_player_id")
                    .table(Guesses::Table)
                    .col(Guesses::PlayerId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guesses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guesses {
    Table,

    Id,
    CreatedAt,
    UpdatedAt,
    ResolvedAt,

    PlayerId,

    Direction,
    EntryPriceScaled,
    ResolvedPriceScaled,
}

#[derive(DeriveIden)]
enum Players {
    Table,
    Id,
}
