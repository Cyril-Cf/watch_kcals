use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "IngredientDetailsTypeEnum"
)]
pub enum IngredientDetailsTypeEnum {
    #[sea_orm(string_value = "ByGrams")]
    ByGrams,
    #[sea_orm(string_value = "ByPiece")]
    ByPiece,
}

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ingredient_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub calories: i32,
    pub proteins: i32,
    pub ingredient_detail_type: IngredientDetailsTypeEnum,
    pub ingredient_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ingredient::Entity",
        from = "Column::IngredientId",
        to = "super::ingredient::Column::Id"
    )]
    Ingredient,
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpsertModel {
    pub calories: i32,
    pub proteins: i32,
    pub ingredient_detail_type: IngredientDetailsTypeEnum,
    pub ingredient_id: Uuid,
}
