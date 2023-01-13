use crate::services::recipe_ingredient_service;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use entity::recipe_ingredient::InsertModel;

#[post("/recipe_ingredients")]
async fn add_recipe_ingredient(
    app_state: web::Data<AppState>,
    create_model: web::Json<InsertModel>,
) -> impl Responder {
    match recipe_ingredient_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(recipe_ingredient) => HttpResponse::Ok().json(recipe_ingredient),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/recipe_ingredients")]
async fn find_all_recipe_ingredients(app_state: web::Data<AppState>) -> impl Responder {
    match recipe_ingredient_service::find_all(&app_state.conn).await {
        Ok(recipe_ingredients) => HttpResponse::Ok().json(recipe_ingredients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
