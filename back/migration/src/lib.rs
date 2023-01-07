pub use sea_orm_migration::*;

mod enums_migrations;
mod ingredient_category_migration;
mod ingredient_detail_migration;
mod ingredient_migration;
mod meal_declaration_ingredient_migration;
mod meal_declaration_migration;
mod recipe_category_migration;
mod recipe_ingredient_migration;
mod recipe_line_migration;
mod recipe_migration;
mod seed_ingredient_category;
mod user_migration;
mod weighing_migration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Tables creation
            Box::new(enums_migrations::Migration),
            Box::new(ingredient_category_migration::Migration),
            Box::new(ingredient_migration::Migration),
            Box::new(ingredient_detail_migration::Migration),
            Box::new(user_migration::Migration),
            Box::new(meal_declaration_migration::Migration),
            Box::new(meal_declaration_ingredient_migration::Migration),
            Box::new(recipe_category_migration::Migration),
            Box::new(recipe_migration::Migration),
            Box::new(recipe_line_migration::Migration),
            Box::new(recipe_ingredient_migration::Migration),
            Box::new(weighing_migration::Migration),
            // Seeding various tables
            Box::new(seed_ingredient_category::Migration),
        ]
    }
}
