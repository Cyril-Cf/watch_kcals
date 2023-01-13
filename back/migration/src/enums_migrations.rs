use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql_requests =
            vec![
                r#"CREATE TYPE IngredientDetailsTypeEnum AS ENUM ('ByGrams', 'ByPiece')"#,
                r#"CREATE TYPE GenderTypeEnum AS ENUM ('Male', 'Female')"#
            ];
        for sql_request in sql_requests {
            let stmt =
                Statement::from_string(manager.get_database_backend(), sql_request.to_owned());
            manager.get_connection().execute(stmt).await.map(|_| ())?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql_requests = vec![
            r#"DROP TYPE IngredientDetailsTypeEnum;"#,
            r#"DROP TYPE GenderTypeEnum;"#
        ];
        for sql_request in sql_requests {
            let stmt =
                Statement::from_string(manager.get_database_backend(), sql_request.to_owned());
            manager.get_connection().execute(stmt).await.map(|_| ())?;
        }
        Ok(())
    }
}
