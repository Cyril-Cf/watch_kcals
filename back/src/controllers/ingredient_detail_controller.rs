use crate::services::ingredient_detail_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::ingredient_detail::UpsertModel;
use uuid::Uuid;

#[delete("/ingredient_details/{ingredient_detail_id}")]
async fn delete_ingredient_detail(
    app_state: web::Data<AppState>,
    ingredient_detail_id: web::Path<String>,
) -> impl Responder {
    match ingredient_detail_id.parse::<Uuid>() {
        Ok(ingredient_detail_id) => {
            match ingredient_detail_service::delete(ingredient_detail_id, &app_state.conn).await {
                Ok(delete_result) => match delete_result {
                    Some(result) => {
                        HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                    }
                    None => HttpResponse::Ok().body("No ingredient_detail deleted"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/ingredient_details/{ingredient_detail_id}")]
async fn update_ingredient_detail(
    app_state: web::Data<AppState>,
    ingredient_detail_id: web::Path<String>,
    ingredient_detail_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match ingredient_detail_id.parse::<Uuid>() {
        Ok(ingredient_detail_id) => {
            match ingredient_detail_service::update(
                &app_state.conn,
                ingredient_detail_id,
                ingredient_detail_from_request.into_inner(),
            )
            .await
            {
                Ok(ingredient_detail_option) => match ingredient_detail_option {
                    Some(ingredient_detail) => HttpResponse::Ok().json(ingredient_detail),
                    None => HttpResponse::NotFound().body("ingredient_detail not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/ingredient_details")]
async fn add_ingredient_detail(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match ingredient_detail_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(ingredient_detail) => HttpResponse::Ok().json(ingredient_detail),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/ingredient_details/{ingredient_detail_id}")]
async fn find_one_ingredient_detail(
    app_state: web::Data<AppState>,
    ingredient_detail_id: web::Path<String>,
) -> impl Responder {
    match ingredient_detail_id.parse::<Uuid>() {
        Ok(ingredient_detail_id) => {
            match ingredient_detail_service::find_one(ingredient_detail_id, &app_state.conn).await {
                Ok(ingredient_detail_option) => match ingredient_detail_option {
                    Some(ingredient_detail) => HttpResponse::Ok().json(ingredient_detail),
                    None => HttpResponse::NotFound().body("ingredient_detail not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/ingredient_details")]
async fn find_all_ingredient_details(app_state: web::Data<AppState>) -> impl Responder {
    match ingredient_detail_service::find_all(&app_state.conn).await {
        Ok(ingredient_details) => HttpResponse::Ok().json(ingredient_details),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/ingredient_details/ingredients/{ingredient_detail_id}")]
async fn find_all_for_ingredient(
    app_state: web::Data<AppState>,
    ingredient_detail_id: web::Path<String>,
) -> impl Responder {
    match ingredient_detail_id.parse::<Uuid>() {
        Ok(ingredient_detail_id) => {
            match ingredient_detail_service::find_all_for_ingredient(
                &app_state.conn,
                ingredient_detail_id,
            )
            .await
            {
                Ok(ingredient_details) => HttpResponse::Ok().json(ingredient_details),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
