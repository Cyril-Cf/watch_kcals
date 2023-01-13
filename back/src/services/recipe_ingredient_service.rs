use entity::recipe_ingredient::{ActiveModel, Entity, InsertModel, Model};
use sea_orm::{entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(conn).await
}

pub async fn create(create_model: InsertModel, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    ActiveModel {
        ingredient_id: ActiveValue::Set(create_model.ingredient_id),
        recipe_id: ActiveValue::Set(create_model.recipe_id),
    }
    .insert(conn)
    .await
}
