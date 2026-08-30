pub use sea_orm_migration::prelude::*;

mod m20260830_135343_create_players_table;
mod m20260830_135452_create_guesses_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260830_135343_create_players_table::Migration),
            Box::new(m20260830_135452_create_guesses_table::Migration),
        ]
    }
}
