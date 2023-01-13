use crate::services::meal_declaration_ingredient_service;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use entity::meal_declaration_ingredient::UpsertModel;

#[post("/meal_declaration_ingredients")]
async fn add_meal_declaration_ingredient(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match meal_declaration_ingredient_service::create(create_model.into_inner(), &app_state.conn)
        .await
    {
        Ok(meal_declaration_ingredient) => HttpResponse::Ok().json(meal_declaration_ingredient),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/meal_declaration_ingredients")]
async fn find_all_meal_declaration_ingredients(app_state: web::Data<AppState>) -> impl Responder {
    match meal_declaration_ingredient_service::find_all(&app_state.conn).await {
        Ok(meal_declaration_ingredients) => HttpResponse::Ok().json(meal_declaration_ingredients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
