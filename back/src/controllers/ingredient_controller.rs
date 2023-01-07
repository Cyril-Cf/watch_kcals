use crate::services::ingredient_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::ingredient::UpsertModel;
use uuid::Uuid;

#[delete("/ingredients/{ingredient_id}")]
async fn delete_ingredient(
    app_state: web::Data<AppState>,
    ingredient_id: web::Path<String>,
) -> impl Responder {
    match ingredient_id.parse::<Uuid>() {
        Ok(ingredient_id) => match ingredient_service::delete(ingredient_id, &app_state.conn).await
        {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::Ok().body("No ingredient deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/ingredients/{ingredient_id}")]
async fn update_ingredient(
    app_state: web::Data<AppState>,
    ingredient_id: web::Path<String>,
    ingredient_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match ingredient_id.parse::<Uuid>() {
        Ok(ingredient_id) => {
            match ingredient_service::update(
                &app_state.conn,
                ingredient_id,
                ingredient_from_request.into_inner(),
            )
            .await
            {
                Ok(ingredient_option) => match ingredient_option {
                    Some(ingredient) => HttpResponse::Ok().json(ingredient),
                    None => HttpResponse::NotFound().body("ingredient not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/ingredients")]
async fn add_ingredient(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match ingredient_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(ingredient) => HttpResponse::Ok().json(ingredient),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/ingredients/{ingredient_id}")]
async fn find_one_ingredient(
    app_state: web::Data<AppState>,
    ingredient_id: web::Path<String>,
) -> impl Responder {
    match ingredient_id.parse::<Uuid>() {
        Ok(ingredient_id) => {
            match ingredient_service::find_one(ingredient_id, &app_state.conn).await {
                Ok(ingredient_option) => match ingredient_option {
                    Some(ingredient) => HttpResponse::Ok().json(ingredient),
                    None => HttpResponse::NotFound().body("ingredient not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/ingredients")]
async fn find_all_ingredients(app_state: web::Data<AppState>) -> impl Responder {
    match ingredient_service::find_all(&app_state.conn).await {
        Ok(ingredients) => HttpResponse::Ok().json(ingredients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
