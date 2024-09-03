use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_managers_table::Managers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Companies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Companies::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Companies::ManagerId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Companies::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Companies::FirstName).string().not_null())
                    .col(ColumnDef::new(Companies::LastName).string().not_null())
                    .col(ColumnDef::new(Companies::Type).tiny_integer().not_null())
                    .col(
                        ColumnDef::new(Companies::Domain)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Companies::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Companies::Status).tiny_integer().not_null())
                    .col(
                        ColumnDef::new(Companies::Credability)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Companies::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("fk-companies_manager_id-managers_id")
                    .to(Managers::Table, Managers::Id)
                    .from(Companies::Table, Companies::ManagerId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Companies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Companies {
    Table,
    Id,
    ManagerId,
    Name,
    FirstName,
    LastName,
    Type,
    Domain,
    Email,
    Status,
    Credability,
    CreatedAt,
}
