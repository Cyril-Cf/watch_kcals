use crate::services::recipe_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::recipe::UpsertModel;
use uuid::Uuid;

#[delete("/recipes/{recipe_id}")]
async fn delete_recipe(
    app_state: web::Data<AppState>,
    recipe_id: web::Path<String>,
) -> impl Responder {
    match recipe_id.parse::<Uuid>() {
        Ok(recipe_id) => match recipe_service::delete(recipe_id, &app_state.conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::Ok().body("No recipe deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/recipes/{recipe_id}")]
async fn update_recipe(
    app_state: web::Data<AppState>,
    recipe_id: web::Path<String>,
    recipe_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match recipe_id.parse::<Uuid>() {
        Ok(recipe_id) => {
            match recipe_service::update(
                &app_state.conn,
                recipe_id,
                recipe_from_request.into_inner(),
            )
            .await
            {
                Ok(recipe_option) => match recipe_option {
                    Some(recipe) => HttpResponse::Ok().json(recipe),
                    None => HttpResponse::NotFound().body("recipe not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/recipes")]
async fn add_recipe(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match recipe_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(recipe) => HttpResponse::Ok().json(recipe),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/recipes/{recipe_id}")]
async fn find_one_recipe(
    app_state: web::Data<AppState>,
    recipe_id: web::Path<String>,
) -> impl Responder {
    match recipe_id.parse::<Uuid>() {
        Ok(recipe_id) => match recipe_service::find_one(recipe_id, &app_state.conn).await {
            Ok(recipe_option) => match recipe_option {
                Some(recipe) => HttpResponse::Ok().json(recipe),
                None => HttpResponse::NotFound().body("recipe not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/recipes")]
async fn find_all_recipes(app_state: web::Data<AppState>) -> impl Responder {
    match recipe_service::find_all(&app_state.conn).await {
        Ok(recipes) => HttpResponse::Ok().json(recipes),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
