use entity::recipe_line::{ActiveModel, Entity, Model, UpsertModel};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait,
};
use uuid::Uuid;

pub async fn find_one(id: Uuid, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(conn).await
}

pub async fn create(create_model: UpsertModel, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        content: ActiveValue::Set(create_model.content),
        order: ActiveValue::Set(create_model.order),
        recipe_id: ActiveValue::Set(create_model.recipe_id),
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
            if user_from_request.content != user_in_db.content {
                let mut active_model = user_in_db.into_active_model();
                active_model.content = ActiveValue::Set(user_from_request.content.to_owned());
                Ok(Some(active_model.update(conn).await?))
            } else {
                Ok(None)
            }
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
