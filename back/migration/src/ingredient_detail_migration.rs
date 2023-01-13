use entity::ingredient_detail::*;
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
                    .col(ColumnDef::new(Column::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Column::Calories).integer().not_null())
                    .col(ColumnDef::new(Column::Proteins).integer().not_null())
                    .col(
                        ColumnDef::new(Column::IngredientDetailType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::IngredientId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("ingredient_details_ingredients")
                            .from(Entity, Column::IngredientId)
                            .to(entity::ingredient::Entity, entity::ingredient::Column::Id),
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
