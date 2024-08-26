use sea_orm_migration::{
    prelude::*,
    schema::{boolean, pk_auto, string, string_null, timestamp_with_time_zone},
};

use crate::m20220101_000001_main::Country;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the approval status table.
        manager
            .create_table(
                Table::create()
                    .table(ApprovalStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApprovalStatus::Status)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        let insert_approval_statuses = Query::insert()
            .into_table(ApprovalStatus::Table)
            .columns([ApprovalStatus::Status])
            .values_panic(["Pending".into()])
            .values_panic(["Approved".into()])
            .values_panic(["Rejected".into()])
            .to_owned();
        manager.exec_stmt(insert_approval_statuses).await?;

        // Create the Submission table.
        manager
            .create_table(
                Table::create()
                    .table(Submission::Table)
                    .if_not_exists()
                    .col(pk_auto(Submission::Id))
                    .col(string(Submission::FirstName))
                    .col(string(Submission::LastName))
                    .col(string_null(Submission::Occupation))
                    .col(string_null(Submission::Organization))
                    .col(string(Submission::Email))
                    .col(string(Submission::Country))
                    .col(string(Submission::City))
                    .col(string_null(Submission::Region))
                    .col(string(Submission::FIPSCode).default("0"))
                    .col(boolean(Submission::Consent).boolean())
                    .col(string(Submission::Status).string())
                    .col(
                        timestamp_with_time_zone(Submission::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submission::Table, Submission::Status)
                            .to(ApprovalStatus::Table, ApprovalStatus::Status),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submission::Table, Submission::Country)
                            .to(Country::Table, Country::Name),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Submission::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Submission {
    Table,
    Id,
    FirstName,
    LastName,
    Occupation,
    Organization,
    Email,
    Country,
    City,
    Region,
    FIPSCode,
    Consent,
    Status,
    CreatedAt,
}

/// Lookup table for the approval statuses.
#[derive(Iden)]
pub enum ApprovalStatus {
    Table,
    Status,
}
