use entity::ingredient_category::ActiveModel as ActiveIntegredientCategory;
use entity::ingredient_category::Entity;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        ActiveIntegredientCategory {
            id: Set(Uuid::new_v4()),
            name: Set("Ready meal".to_owned()),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for model in Entity::find().all(db).await? {
            model.delete(db).await?;
        }
        Ok(())
    }
}
