use entity::meal_declaration::*;
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
                    .col(ColumnDef::new(Column::Date).date().not_null())
                    .col(ColumnDef::new(Column::TimeOfConsumption).time())
                    .col(ColumnDef::new(Column::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("meal_declarations_users")
                            .from(Entity, Column::UserId)
                            .to(entity::user::Entity, entity::user::Column::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Entity).to_owned())
            .await
    }
}
