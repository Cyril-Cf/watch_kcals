use crate::services::ingredient_category_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::ingredient_category::UpsertModel;
use uuid::Uuid;

#[delete("/ingredient_categories/{ingredient_category_id}")]
async fn delete_ingredient_category(
    app_state: web::Data<AppState>,
    ingredient_category_id: web::Path<String>,
) -> impl Responder {
    match ingredient_category_id.parse::<Uuid>() {
        Ok(ingredient_category_id) => {
            match ingredient_category_service::delete(ingredient_category_id, &app_state.conn).await
            {
                Ok(delete_result) => match delete_result {
                    Some(result) => {
                        HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                    }
                    None => HttpResponse::Ok().body("No ingredient_category deleted"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/ingredient_categories/{ingredient_category_id}")]
async fn update_ingredient_category(
    app_state: web::Data<AppState>,
    ingredient_category_id: web::Path<String>,
    ingredient_category_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match ingredient_category_id.parse::<Uuid>() {
        Ok(ingredient_category_id) => {
            match ingredient_category_service::update(
                &app_state.conn,
                ingredient_category_id,
                ingredient_category_from_request.into_inner(),
            )
            .await
            {
                Ok(ingredient_category_option) => match ingredient_category_option {
                    Some(ingredient_category) => HttpResponse::Ok().json(ingredient_category),
                    None => HttpResponse::NotFound().body("ingredient_category not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/ingredient_categories")]
async fn add_ingredient_category(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match ingredient_category_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(ingredient_category) => HttpResponse::Ok().json(ingredient_category),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/ingredient_categories/{ingredient_category_id}")]
async fn find_one_ingredient_category(
    app_state: web::Data<AppState>,
    ingredient_category_id: web::Path<String>,
) -> impl Responder {
    match ingredient_category_id.parse::<Uuid>() {
        Ok(ingredient_category_id) => {
            match ingredient_category_service::find_one(ingredient_category_id, &app_state.conn)
                .await
            {
                Ok(ingredient_category_option) => match ingredient_category_option {
                    Some(ingredient_category) => HttpResponse::Ok().json(ingredient_category),
                    None => HttpResponse::NotFound().body("ingredient_category not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/ingredient_categories")]
async fn find_all_ingredient_categories(app_state: web::Data<AppState>) -> impl Responder {
    match ingredient_category_service::find_all(&app_state.conn).await {
        Ok(ingredient_categories) => HttpResponse::Ok().json(ingredient_categories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
