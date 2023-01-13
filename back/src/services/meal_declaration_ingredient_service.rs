use entity::meal_declaration_ingredient::{ActiveModel, Entity, Model, UpsertModel};
use sea_orm::{entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(conn).await
}

pub async fn create(create_model: UpsertModel, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    ActiveModel {
        ingredient_detail_id: ActiveValue::Set(create_model.ingredient_detail_id),
        ingredient_id: ActiveValue::Set(create_model.ingredient_id),
        meal_declaration_id: ActiveValue::Set(create_model.meal_declaration_id),
        quantity: ActiveValue::Set(create_model.quantity),
    }
    .insert(conn)
    .await
}
