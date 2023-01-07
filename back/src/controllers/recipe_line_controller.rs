use crate::services::recipe_line_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use entity::recipe_line::UpsertModel;
use uuid::Uuid;

#[delete("/recipe_lines/{recipe_line_id}")]
async fn delete_recipe_line(
    app_state: web::Data<AppState>,
    recipe_line_id: web::Path<String>,
) -> impl Responder {
    match recipe_line_id.parse::<Uuid>() {
        Ok(recipe_line_id) => {
            match recipe_line_service::delete(recipe_line_id, &app_state.conn).await {
                Ok(delete_result) => match delete_result {
                    Some(result) => {
                        HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                    }
                    None => HttpResponse::Ok().body("No recipe_line deleted"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/recipe_lines/{recipe_line_id}")]
async fn update_recipe_line(
    app_state: web::Data<AppState>,
    recipe_line_id: web::Path<String>,
    recipe_line_from_request: web::Json<UpsertModel>,
) -> impl Responder {
    match recipe_line_id.parse::<Uuid>() {
        Ok(recipe_line_id) => {
            match recipe_line_service::update(
                &app_state.conn,
                recipe_line_id,
                recipe_line_from_request.into_inner(),
            )
            .await
            {
                Ok(recipe_line_option) => match recipe_line_option {
                    Some(recipe_line) => HttpResponse::Ok().json(recipe_line),
                    None => HttpResponse::NotFound().body("recipe_line not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/recipe_lines")]
async fn add_recipe_line(
    app_state: web::Data<AppState>,
    create_model: web::Json<UpsertModel>,
) -> impl Responder {
    match recipe_line_service::create(create_model.into_inner(), &app_state.conn).await {
        Ok(recipe_line) => HttpResponse::Ok().json(recipe_line),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/recipe_lines/{recipe_line_id}")]
async fn find_one_recipe_line(
    app_state: web::Data<AppState>,
    recipe_line_id: web::Path<String>,
) -> impl Responder {
    match recipe_line_id.parse::<Uuid>() {
        Ok(recipe_line_id) => {
            match recipe_line_service::find_one(recipe_line_id, &app_state.conn).await {
                Ok(recipe_line_option) => match recipe_line_option {
                    Some(recipe_line) => HttpResponse::Ok().json(recipe_line),
                    None => HttpResponse::NotFound().body("recipe_line not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/recipe_lines")]
async fn find_all_recipe_lines(app_state: web::Data<AppState>) -> impl Responder {
    match recipe_line_service::find_all(&app_state.conn).await {
        Ok(recipe_lines) => HttpResponse::Ok().json(recipe_lines),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
