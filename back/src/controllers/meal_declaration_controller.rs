use crate::services::meal_declaration_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::meal_declaration::UpsertModel;
use uuid::Uuid;

#[delete("/meal_declarations/{meal_declaration_id}")]
async fn delete_meal_declaration(
    app_state: web::Data<AppState>,
    meal_declaration_id: web::Path<String>,
) -> impl Responder {
    match meal_declaration_id.parse::<Uuid>() {
        Ok(meal_declaration_id) => {
            match meal_declaration_service::delete(meal_declaration_id, &app_state.conn).await {
                Ok(delete_result) => match delete_result {
                    Some(result) => {
                        HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                    }
                    None => HttpResponse::Ok().body("No meal_declaration deleted"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/meal_declarations/{meal_declaration_id}")]
async fn update_meal_declaration(
    app_state: web::Data<AppState>,
    meal_declaration_id: web::Path<String>,
    meal_declaration_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match meal_declaration_id.parse::<Uuid>() {
        Ok(meal_declaration_id) => {
            match meal_declaration_service::update(
                &app_state.conn,
                meal_declaration_id,
                meal_declaration_from_request.into_inner(),
            )
            .await
            {
                Ok(meal_declaration_option) => match meal_declaration_option {
                    Some(meal_declaration) => HttpResponse::Ok().json(meal_declaration),
                    None => HttpResponse::NotFound().body("meal_declaration not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/meal_declarations")]
async fn add_meal_declaration(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match meal_declaration_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(meal_declaration) => HttpResponse::Ok().json(meal_declaration),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/meal_declarations/{meal_declaration_id}")]
async fn find_one_meal_declaration(
    app_state: web::Data<AppState>,
    meal_declaration_id: web::Path<String>,
) -> impl Responder {
    match meal_declaration_id.parse::<Uuid>() {
        Ok(meal_declaration_id) => {
            match meal_declaration_service::find_one(meal_declaration_id, &app_state.conn).await {
                Ok(meal_declaration_option) => match meal_declaration_option {
                    Some(meal_declaration) => HttpResponse::Ok().json(meal_declaration),
                    None => HttpResponse::NotFound().body("meal_declaration not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/meal_declarations")]
async fn find_all_meal_declarations(app_state: web::Data<AppState>) -> impl Responder {
    match meal_declaration_service::find_all(&app_state.conn).await {
        Ok(meal_declarations) => HttpResponse::Ok().json(meal_declarations),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
