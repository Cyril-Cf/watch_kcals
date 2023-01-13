use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "GenderTypeEnum"
)]
pub enum GenderTypeEnum {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
}

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub gender: GenderTypeEnum,
    pub date_of_birth: Date,
    pub height: i32,
    pub physical_activity_level: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::weighing::Entity")]
    Weighing,
    #[sea_orm(has_many = "super::meal_declaration::Entity")]
    MealDeclaration,
}

impl Related<super::weighing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Weighing.def()
    }
}

impl Related<super::meal_declaration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MealDeclaration.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpsertModel {
    pub name: String,
    pub gender: GenderTypeEnum,
    pub date_of_birth: Date,
    pub height: i32,
    pub physical_activity_level: i32,
}
