// To use when deploying the application with shuttle

// use actix_cors::Cors;
// use actix_web::middleware::Logger;
// use actix_web::{web, web::ServiceConfig};
// use migration::{Migrator, MigratorTrait};
// use sea_orm::DatabaseConnection;
// use shuttle_service::ShuttleActixWeb;
// use sqlx::PgPool;

// To run during development inside the container
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::env;
use std::time::Duration;
// use env_logger::Env;

mod controllers;
mod services;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

// To run during development inside the container

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    // env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init_from_env(Env::default().default_filter_or(
    //     std::env::var("ENV_LOGGER_FILTER").expect("ENV_LOGGER_FILTER is not set in .env file"),
    // ));

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    println!("Connexion to db_url : {}", db_url);
    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let conn = sea_orm::Database::connect(opt).await.unwrap();
    println!("Connexion successful!");
    println!("Applying migrations...");
    // Migrator::down(&conn, None).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    println!("Applying migrations... successful!");
    let state = AppState { conn };
    println!("App launched!");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(web::scope("/api").wrap(Cors::permissive()).configure(init))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

// To use when deploying the application with shuttle

// #[shuttle_service::main]
// async fn actix_web(
//     #[shuttle_shared_db::Postgres] pool: PgPool,
// ) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
//     let conn: sea_orm::DatabaseConnection =
//         sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
//     Migrator::up(&conn, None).await.unwrap();
//     let state = AppState { conn };
//     Ok(move |cfg: &mut ServiceConfig| {
//         cfg.service(
//             web::scope("/api")
//                 .wrap(Cors::permissive())
//                 .wrap(Logger::default())
//                 .app_data(web::Data::new(state.clone()))
//                 .configure(init),
//         );
//     })
// }

pub fn init(cfg: &mut web::ServiceConfig) {
    // ingredient categories
    cfg.service(controllers::ingredient_category_controller::add_ingredient_category);
    cfg.service(controllers::ingredient_category_controller::find_one_ingredient_category);
    cfg.service(controllers::ingredient_category_controller::find_all_ingredient_categories);
    cfg.service(controllers::ingredient_category_controller::update_ingredient_category);
    cfg.service(controllers::ingredient_category_controller::delete_ingredient_category);

    // ingredient details
    cfg.service(controllers::ingredient_detail_controller::add_ingredient_detail);
    cfg.service(controllers::ingredient_detail_controller::find_one_ingredient_detail);
    cfg.service(controllers::ingredient_detail_controller::find_all_ingredient_details);
    cfg.service(controllers::ingredient_detail_controller::update_ingredient_detail);
    cfg.service(controllers::ingredient_detail_controller::delete_ingredient_detail);

    // ingredients
    cfg.service(controllers::ingredient_controller::add_ingredient);
    cfg.service(controllers::ingredient_controller::find_one_ingredient);
    cfg.service(controllers::ingredient_controller::find_all_ingredients);
    cfg.service(controllers::ingredient_controller::update_ingredient);
    cfg.service(controllers::ingredient_controller::delete_ingredient);

    // meal declaration ingredients
    cfg.service(
        controllers::meal_declaration_ingredient_controller::add_meal_declaration_ingredient,
    );
    cfg.service(
        controllers::meal_declaration_ingredient_controller::find_all_meal_declaration_ingredients,
    );

    // meal declarations
    cfg.service(controllers::meal_declaration_controller::add_meal_declaration);
    cfg.service(controllers::meal_declaration_controller::find_one_meal_declaration);
    cfg.service(controllers::meal_declaration_controller::find_all_meal_declarations);
    cfg.service(controllers::meal_declaration_controller::update_meal_declaration);
    cfg.service(controllers::meal_declaration_controller::delete_meal_declaration);

    // recipe categories
    cfg.service(controllers::recipe_category_controller::add_recipe_category);
    cfg.service(controllers::recipe_category_controller::find_one_recipe_category);
    cfg.service(controllers::recipe_category_controller::find_all_recipe_categories);
    cfg.service(controllers::recipe_category_controller::update_recipe_category);
    cfg.service(controllers::recipe_category_controller::delete_recipe_category);

    // recipe ingredients
    cfg.service(controllers::recipe_ingregient_controller::add_recipe_ingredient);
    cfg.service(controllers::recipe_ingregient_controller::find_all_recipe_ingredients);

    // recipe lines
    cfg.service(controllers::recipe_line_controller::add_recipe_line);
    cfg.service(controllers::recipe_line_controller::find_one_recipe_line);
    cfg.service(controllers::recipe_line_controller::find_all_recipe_lines);
    cfg.service(controllers::recipe_line_controller::update_recipe_line);
    cfg.service(controllers::recipe_line_controller::delete_recipe_line);

    // recipes
    cfg.service(controllers::recipe_controller::add_recipe);
    cfg.service(controllers::recipe_controller::find_one_recipe);
    cfg.service(controllers::recipe_controller::find_all_recipes);
    cfg.service(controllers::recipe_controller::update_recipe);
    cfg.service(controllers::recipe_controller::delete_recipe);

    // users
    cfg.service(controllers::user_controller::add_user);
    cfg.service(controllers::user_controller::find_one_user);
    cfg.service(controllers::user_controller::find_all_users);
    cfg.service(controllers::user_controller::update_user);
    cfg.service(controllers::user_controller::delete_user);

    // weighings
    cfg.service(controllers::weighing_controller::add_weighing);
    cfg.service(controllers::weighing_controller::find_one_weighing);
    cfg.service(controllers::weighing_controller::find_all_weighings);
    cfg.service(controllers::weighing_controller::update_weighing);
    cfg.service(controllers::weighing_controller::delete_weighing);
}
