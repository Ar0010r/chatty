use sea_orm::sqlx::types::chrono::Utc;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Managers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Managers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Managers::Login)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Managers::Password).string().not_null())
                    .col(ColumnDef::new(Managers::Role).tiny_integer().not_null())
                    .col(ColumnDef::new(Managers::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Managers::Table)
            .columns([
                Managers::Login,
                Managers::Password,
                Managers::Role,
                Managers::CreatedAt,
            ])
            .values_panic([
                "Admin".into(),
                "$2b$12$fXyZDbtveDoZWQLa.MTKReB8w35wmmoMQ2QfI1uFUAbvUScYwiYNG".into(),
                5.into(),
                Utc::now().naive_utc().into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Managers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Managers {
    Table,
    Id,
    Login,
    Password,
    Role,
    CreatedAt,
}
