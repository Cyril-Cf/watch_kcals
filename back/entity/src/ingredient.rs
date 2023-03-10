use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ingredients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub ingredient_category_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ingredient_detail::Entity")]
    IngredientDetails,
    #[sea_orm(has_many = "super::meal_declaration_ingredient::Entity")]
    MealDeclaration,
    #[sea_orm(has_many = "super::recipe_ingredient::Entity")]
    Recipe,
    #[sea_orm(
        belongs_to = "super::ingredient_category::Entity",
        from = "Column::IngredientCategoryId",
        to = "super::ingredient_category::Column::Id"
    )]
    IngredientCategory,
}

impl Related<super::ingredient_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IngredientDetails.def()
    }
}

impl Related<super::ingredient_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IngredientCategory.def()
    }
}

impl Related<super::meal_declaration::Entity> for Entity {
    fn to() -> RelationDef {
        super::meal_declaration_ingredient::Relation::MealDeclaration.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::meal_declaration_ingredient::Relation::Ingredient
                .def()
                .rev(),
        )
    }
}

impl Related<super::recipe::Entity> for Entity {
    fn to() -> RelationDef {
        super::recipe_ingredient::Relation::Recipe.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::recipe_ingredient::Relation::Ingredient.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpsertModel {
    pub name: String,
    pub ingredient_category_id: Uuid,
}
