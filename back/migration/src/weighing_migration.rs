use entity::weighing::*;
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
                    .col(ColumnDef::new(Column::Weight).float().not_null())
                    .col(ColumnDef::new(Column::BodyFatPercentage).integer())
                    .col(ColumnDef::new(Column::WaistCircumference).integer())
                    .col(ColumnDef::new(Column::WaistSize).integer())
                    .col(ColumnDef::new(Column::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("weighings_users")
                            .from(Entity, Column::UserId)
                            .to(entity::user::Entity, entity::user::Column::Id),
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
