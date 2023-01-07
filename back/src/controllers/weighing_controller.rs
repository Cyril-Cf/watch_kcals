use crate::services::weighing_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::weighing::UpsertModel;
use uuid::Uuid;

#[delete("/weighings/{weighing_id}")]
async fn delete_weighing(
    app_state: web::Data<AppState>,
    weighing_id: web::Path<String>,
) -> impl Responder {
    match weighing_id.parse::<Uuid>() {
        Ok(weighing_id) => match weighing_service::delete(weighing_id, &app_state.conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::Ok().body("No weighing deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/weighings/{weighing_id}")]
async fn update_weighing(
    app_state: web::Data<AppState>,
    weighing_id: web::Path<String>,
    weighing_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match weighing_id.parse::<Uuid>() {
        Ok(weighing_id) => {
            match weighing_service::update(
                &app_state.conn,
                weighing_id,
                weighing_from_request.into_inner(),
            )
            .await
            {
                Ok(weighing_option) => match weighing_option {
                    Some(weighing) => HttpResponse::Ok().json(weighing),
                    None => HttpResponse::NotFound().body("weighing not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/weighings")]
async fn add_weighing(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match weighing_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(weighing) => HttpResponse::Ok().json(weighing),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/weighings/{weighing_id}")]
async fn find_one_weighing(
    app_state: web::Data<AppState>,
    weighing_id: web::Path<String>,
) -> impl Responder {
    match weighing_id.parse::<Uuid>() {
        Ok(weighing_id) => match weighing_service::find_one(weighing_id, &app_state.conn).await {
            Ok(weighing_option) => match weighing_option {
                Some(weighing) => HttpResponse::Ok().json(weighing),
                None => HttpResponse::NotFound().body("weighing not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/weighings")]
async fn find_all_weighings(app_state: web::Data<AppState>) -> impl Responder {
    match weighing_service::find_all(&app_state.conn).await {
        Ok(weighings) => HttpResponse::Ok().json(weighings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
