use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,

    pub score: i32,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::guesses::Entity")]
    Guesses,
}

impl Related<super::guesses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guesses.def()
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
