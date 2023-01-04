use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "meal_declarations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub date: Date,
    pub user_id: i32,
    pub time_of_consumption: Time,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::meal_declaration_ingredient::Entity")]
    Ingredient,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        super::meal_declaration_ingredient::Relation::Ingredient.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::meal_declaration_ingredient::Relation::MealDeclaration
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateModel {
    pub date: Date,
    pub time_of_consumption: Time,
}
