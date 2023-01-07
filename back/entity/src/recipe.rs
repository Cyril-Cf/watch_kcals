use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "recipes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub difficulty_ranking: u32,
    pub total_time: u32,
    pub recipe_category_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ingredient::Entity")]
    Ingredient,
    #[sea_orm(
        belongs_to = "super::recipe_category::Entity",
        from = "Column::RecipeCategoryId",
        to = "super::recipe_category::Column::Id"
    )]
    RecipeCategory,
    #[sea_orm(has_many = "super::recipe_line::Entity")]
    RecipeLine,
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        super::recipe_ingredient::Relation::Ingredient.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::recipe_ingredient::Relation::Recipe.def().rev())
    }
}

impl Related<super::recipe_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeCategory.def()
    }
}

impl Related<super::recipe_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeLine.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpsertModel {
    pub name: String,
    pub difficulty_ranking: u32,
    pub total_time: u32,
    pub recipe_category_id: Uuid,
}
