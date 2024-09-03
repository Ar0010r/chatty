use sea_orm_migration::prelude::*;

use crate::{
    m20220101_000001_create_managers_table::Managers,
    m20220101_000002_create_companies_table::Companies,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, lead: &SchemaManager) -> Result<(), DbErr> {
        lead.create_table(
            Table::create()
                .table(Leads::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Leads::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Leads::HrCompanyId).big_integer().null())
                .col(ColumnDef::new(Leads::CompanyId).big_integer().null())
                .col(ColumnDef::new(Leads::HrId).big_integer().null())
                .col(ColumnDef::new(Leads::FirstName).string().null())
                .col(ColumnDef::new(Leads::LastName).string().null())
                .col(
                    ColumnDef::new(Leads::Status)
                        .tiny_integer()
                        .not_null()
                        .default(0),
                )
                .col(
                    ColumnDef::new(Leads::Emails)
                        .array(ColumnType::Array(RcOrArc::new(ColumnType::String(
                            StringLen::N(200),
                        ))))
                        .not_null(),
                )
                .col(ColumnDef::new(Leads::CreatedAt).date_time().not_null())
                .to_owned(),
        )
        .await?;

        lead.create_foreign_key(
            sea_query::ForeignKey::create()
                .name("fk-leads_hr_id-managers_id")
                .to(Managers::Table, Managers::Id)
                .from(Leads::Table, Leads::HrId)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction)
                .to_owned(),
        )
        .await?;

        lead.create_foreign_key(
            sea_query::ForeignKey::create()
                .name("fk-leads_hr_company_id-managers_id")
                .to(Companies::Table, Companies::Id)
                .from(Leads::Table, Leads::HrCompanyId)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction)
                .to_owned(),
        )
        .await?;

        lead.create_foreign_key(
            sea_query::ForeignKey::create()
                .name("fk-leads_company_id-managers_id")
                .to(Companies::Table, Companies::Id)
                .from(Leads::Table, Leads::CompanyId)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction)
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Leads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Leads {
    Table,
    Id,
    HrCompanyId,
    CompanyId,
    HrId,
    Emails,
    FirstName,
    LastName,
    Status,
    CreatedAt,
}
