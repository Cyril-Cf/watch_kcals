use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub is_woman: bool,
    pub date_of_birth: Date,
    pub height: u32,
    pub physical_activity_level: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::weighing::Entity")]
    Weighing,
}

impl Related<super::weighing::Entity> for Entity {
    fn to() -> RelationDef {
        super::weighing::Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateModel {
    pub name: String,
    pub weight: f32,
    pub is_woman: bool,
    pub date_of_birth: Date,
    pub height: u32,
    pub physical_activity_level: u32,
}
