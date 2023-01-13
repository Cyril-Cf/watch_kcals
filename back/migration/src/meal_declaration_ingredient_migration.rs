use entity::meal_declaration_ingredient::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Quantity).integer().not_null())
                    .col(ColumnDef::new(Column::MealDeclarationId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("meal_declaration_ingredients_meal_declarations")
                            .from(Entity, Column::MealDeclarationId)
                            .to(
                                entity::meal_declaration::Entity,
                                entity::meal_declaration::Column::Id,
                            ),
                    )
                    .col(ColumnDef::new(Column::IngredientId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("meal_declaration_ingredients_ingredients")
                            .from(Entity, Column::IngredientId)
                            .to(entity::ingredient::Entity, entity::ingredient::Column::Id),
                    )
                    .col(ColumnDef::new(Column::IngredientDetailId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("meal_declaration_ingredients_ingredient_details")
                            .from(Entity, Column::IngredientDetailId)
                            .to(
                                entity::ingredient_detail::Entity,
                                entity::ingredient_detail::Column::Id,
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Entity).cascade().to_owned())
            .await
    }
}
