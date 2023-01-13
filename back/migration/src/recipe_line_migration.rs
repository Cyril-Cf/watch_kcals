use entity::recipe_line::*;
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
                    .col(ColumnDef::new(Column::Content).string().not_null())
                    .col(ColumnDef::new(Column::Order).integer().not_null())
                    .col(ColumnDef::new(Column::RecipeId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("recipes_lines_recipes")
                            .from(Entity, Column::RecipeId)
                            .to(entity::recipe::Entity, entity::recipe::Column::Id),
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
