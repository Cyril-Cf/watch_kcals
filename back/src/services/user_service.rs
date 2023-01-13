use entity::user::{ActiveModel, Entity, Model, UpsertModel};
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
        name: ActiveValue::Set(create_model.name),
        gender: ActiveValue::Set(create_model.gender),
        date_of_birth: ActiveValue::Set(create_model.date_of_birth),
        height: ActiveValue::Set(create_model.height),
        physical_activity_level: ActiveValue::Set(create_model.physical_activity_level),
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
            active_model.name = ActiveValue::Set(user_from_request.name.to_owned());
            active_model.gender = ActiveValue::Set(user_from_request.gender.to_owned());
            active_model.date_of_birth = ActiveValue::Set(user_from_request.date_of_birth.to_owned());
            active_model.height = ActiveValue::Set(user_from_request.height.to_owned());
            active_model.physical_activity_level = ActiveValue::Set(user_from_request.physical_activity_level.to_owned());
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
