use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "meal_declarations_ingredients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub meal_declaration_id: Uuid,
    #[sea_orm(primary_key)]
    pub ingredient_id: Uuid,
    pub ingredient_detail_id: Uuid,
    pub quantity: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ingredient::Entity",
        from = "Column::IngredientId",
        to = "super::ingredient::Column::Id"
    )]
    Ingredient,
    #[sea_orm(
        belongs_to = "super::meal_declaration::Entity",
        from = "Column::MealDeclarationId",
        to = "super::meal_declaration::Column::Id"
    )]
    MealDeclaration,
    #[sea_orm(
        belongs_to = "super::ingredient_detail::Entity",
        from = "Column::IngredientDetailId",
        to = "super::ingredient_detail::Column::Id"
    )]
    IngredientDetail,
}

impl Related<super::meal_declaration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MealDeclaration.def()
    }
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredient.def()
    }
}

impl Related<super::ingredient_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IngredientDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateModel {
    pub quantity: u32,
}
