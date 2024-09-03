use sea_orm_migration::prelude::*;

use crate::m20220101_000003_create_leads_table::Leads;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, lead: &SchemaManager) -> Result<(), DbErr> {
        lead.create_table(
            Table::create()
                .table(LeadLog::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(LeadLog::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(LeadLog::LeadId).big_integer().not_null())
                .col(
                    ColumnDef::new(LeadLog::Action)
                        .tiny_integer()
                        .not_null()
                        .default(0),
                )
                .col(ColumnDef::new(LeadLog::CreatedAt).date_time().not_null())
                .to_owned(),
        )
        .await?;

        lead.create_foreign_key(
            sea_query::ForeignKey::create()
                .name("fk-lead_log-lead_id")
                .to(Leads::Table, Leads::Id)
                .from(LeadLog::Table, LeadLog::LeadId)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction)
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LeadLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LeadLog {
    Table,
    Id,
    LeadId,
    Action,
    CreatedAt,
}
