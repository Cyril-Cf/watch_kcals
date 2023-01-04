use entity::user::{Entity, Model, UpdateModel};
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

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    entity::user::ActiveModel {
        name: ActiveValue::Set(new_name),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn update(
    conn: &DatabaseConnection,
    id: Uuid,
    user_from_request: UpdateModel,
) -> Result<Option<Model>, DbErr> {
    match find_one(id, conn).await? {
        Some(user_in_db) => {
            if user_from_request.name != user_in_db.name {
                let mut active_model = user_in_db.into_active_model();
                active_model.name = ActiveValue::Set(user_from_request.name.to_owned());
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
