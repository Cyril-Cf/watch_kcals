use entity::weighing::{ActiveModel, Entity, Model, UpsertModel};
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
        body_fat_percentage: ActiveValue::Set(create_model.body_fat_percentage),
        date: ActiveValue::Set(create_model.date),
        user_id: ActiveValue::Set(create_model.user_id),
        waist_circumference: ActiveValue::Set(create_model.waist_circumference),
        waist_size: ActiveValue::Set(create_model.waist_size),
        weight: ActiveValue::Set(create_model.weight),
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
            if user_from_request.body_fat_percentage != user_in_db.body_fat_percentage {
                let mut active_model = user_in_db.into_active_model();
                active_model.body_fat_percentage =
                    ActiveValue::Set(user_from_request.body_fat_percentage.to_owned());
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
