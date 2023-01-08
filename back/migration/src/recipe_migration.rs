use entity::recipe::*;
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
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(
                        ColumnDef::new(Column::DifficultyRanking)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::TotalTime).integer().not_null())
                    .col(ColumnDef::new(Column::RecipeCategoryId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("recipes_recipe_categories")
                            .from(Entity, Column::RecipeCategoryId)
                            .to(
                                entity::recipe_category::Entity,
                                entity::recipe_category::Column::Id,
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
