use crate::services::user_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::user::UpsertModel;
use uuid::Uuid;

#[delete("/users/{user_id}")]
async fn delete_user(app_state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    match user_id.parse::<Uuid>() {
        Ok(user_id) => match user_service::delete(user_id, &app_state.conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::Ok().body("No user deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/users/{user_id}")]
async fn update_user(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
    user_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match user_id.parse::<Uuid>() {
        Ok(user_id) => {
            match user_service::update(&app_state.conn, user_id, user_from_request.into_inner())
                .await
            {
                Ok(user_option) => match user_option {
                    Some(user) => HttpResponse::Ok().json(user),
                    None => HttpResponse::NotFound().body("User not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/users")]
async fn add_user(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match user_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{user_id}")]
async fn find_one_user(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    match user_id.parse::<Uuid>() {
        Ok(user_id) => match user_service::find_one(user_id, &app_state.conn).await {
            Ok(user_option) => match user_option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().body("User not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/users")]
async fn find_all_users(app_state: web::Data<AppState>) -> impl Responder {
    match user_service::find_all(&app_state.conn).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
