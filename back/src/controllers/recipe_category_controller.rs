use crate::services::recipe_category_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::recipe_category::UpsertModel;
use uuid::Uuid;

#[delete("/recipe_categories/{recipe_category_id}")]
async fn delete_recipe_category(
    app_state: web::Data<AppState>,
    recipe_category_id: web::Path<String>,
) -> impl Responder {
    match recipe_category_id.parse::<Uuid>() {
        Ok(recipe_category_id) => {
            match recipe_category_service::delete(recipe_category_id, &app_state.conn).await {
                Ok(delete_result) => match delete_result {
                    Some(result) => {
                        HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                    }
                    None => HttpResponse::Ok().body("No recipe_category deleted"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/recipe_categories/{recipe_category_id}")]
async fn update_recipe_category(
    app_state: web::Data<AppState>,
    recipe_category_id: web::Path<String>,
    recipe_category_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match recipe_category_id.parse::<Uuid>() {
        Ok(recipe_category_id) => {
            match recipe_category_service::update(
                &app_state.conn,
                recipe_category_id,
                recipe_category_from_request.into_inner(),
            )
            .await
            {
                Ok(recipe_category_option) => match recipe_category_option {
                    Some(recipe_category) => HttpResponse::Ok().json(recipe_category),
                    None => HttpResponse::NotFound().body("recipe_category not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/recipe_categories")]
async fn add_recipe_category(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match recipe_category_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(recipe_category) => HttpResponse::Ok().json(recipe_category),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/recipe_categories/{recipe_category_id}")]
async fn find_one_recipe_category(
    app_state: web::Data<AppState>,
    recipe_category_id: web::Path<String>,
) -> impl Responder {
    match recipe_category_id.parse::<Uuid>() {
        Ok(recipe_category_id) => {
            match recipe_category_service::find_one(recipe_category_id, &app_state.conn).await {
                Ok(recipe_category_option) => match recipe_category_option {
                    Some(recipe_category) => HttpResponse::Ok().json(recipe_category),
                    None => HttpResponse::NotFound().body("recipe_category not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/recipe_categories")]
async fn find_all_recipe_categories(app_state: web::Data<AppState>) -> impl Responder {
    match recipe_category_service::find_all(&app_state.conn).await {
        Ok(recipe_categories) => HttpResponse::Ok().json(recipe_categories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
