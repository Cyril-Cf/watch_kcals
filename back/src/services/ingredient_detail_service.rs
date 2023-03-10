use entity::ingredient_detail::{ActiveModel, Column, Entity, Model, UpsertModel};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, DeleteResult,
    EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
};
use uuid::Uuid;

pub async fn find_one(id: Uuid, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(conn).await
}

pub async fn find_all_for_ingredient(
    conn: &DatabaseConnection,
    ingredient_id: Uuid,
) -> Result<Vec<Model>, DbErr> {
    Entity::find()
        .filter(Column::IngredientId.eq(ingredient_id))
        .all(conn)
        .await
}

pub async fn create(create_model: UpsertModel, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        calories: ActiveValue::Set(create_model.calories),
        ingredient_detail_type: ActiveValue::Set(create_model.ingredient_detail_type),
        ingredient_id: ActiveValue::Set(create_model.ingredient_id),
        proteins: ActiveValue::Set(create_model.proteins),
    }
    .insert(conn)
    .await
}

pub async fn update(
    conn: &DatabaseConnection,
    id: Uuid,
    user_from_request: UpsertModel,
) -> Result<Option<Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(user_in_db) => {
            let mut active_model = user_in_db.into_active_model();
            active_model.calories = ActiveValue::Set(user_from_request.calories.to_owned());
            active_model.ingredient_detail_type =
                ActiveValue::Set(user_from_request.ingredient_detail_type.to_owned());
            active_model.proteins = ActiveValue::Set(user_from_request.proteins.to_owned());
            Ok(Some(active_model.update(conn).await?))
        }
        None => Ok(None),
    }
}

pub async fn delete(id: Uuid, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}
