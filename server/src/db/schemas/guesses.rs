use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "guesses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub resolved_at: Option<DateTimeWithTimeZone>,

    pub player_id: Uuid,

    pub direction: GuessDirection,

    pub entry_price_scaled: i64,
    pub resolved_price_scaled: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "camelCase"
)]
pub enum GuessDirection {
    Up,
    Down,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::players::Entity",
        from = "Column::PlayerId",
        to = "super::players::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Player,
}

impl Related<super::players::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now().into();

        self.updated_at = Set(now);

        if insert {
            self.created_at = Set(now);
            return Ok(self);
        }

        Ok(self)
    }
}
