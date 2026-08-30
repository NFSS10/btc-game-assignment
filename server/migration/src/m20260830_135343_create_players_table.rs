use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Players::Table)
                    .if_not_exists()
                    // id: Uuid
                    .col(ColumnDef::new(Players::Id).uuid().not_null().primary_key())
                    // created_at
                    .col(timestamp_with_time_zone(Players::CreatedAt).not_null())
                    // updated_at
                    .col(timestamp_with_time_zone(Players::UpdatedAt).not_null())
                    // score: i32
                    .col(integer(Players::Score).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Players::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Players {
    Table,

    Id,
    CreatedAt,
    UpdatedAt,

    Score,
}
