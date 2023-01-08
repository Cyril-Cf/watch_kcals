use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "weighings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub date: Date,
    pub weight: f32,
    pub body_fat_percentage: Option<i32>,
    pub waist_circumference: Option<i32>,
    pub waist_size: Option<i32>,
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpsertModel {
    pub date: Date,
    pub weight: f32,
    pub body_fat_percentage: Option<i32>,
    pub waist_circumference: Option<i32>,
    pub waist_size: Option<i32>,
    pub user_id: Uuid,
}
