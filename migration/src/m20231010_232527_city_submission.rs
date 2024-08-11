use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Submission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Submission::FirstName).string().not_null())
                    .col(ColumnDef::new(Submission::LastName).string().not_null())
                    .col(ColumnDef::new(Submission::Occupation).string())
                    .col(ColumnDef::new(Submission::Organization).string())
                    .col(ColumnDef::new(Submission::Email).string().not_null())
                    .col(ColumnDef::new(Submission::Country).string().not_null())
                    .col(ColumnDef::new(Submission::City).string().not_null())
                    .col(ColumnDef::new(Submission::Region).string())
                    .col(
                        ColumnDef::new(Submission::FIPSCode)
                            .string()
                            .not_null()
                            .default("0"),
                    )
                    .col(ColumnDef::new(Submission::Consent).boolean().not_null())
                    .col(ColumnDef::new(Submission::Status).string().not_null())
                    .col(
                        ColumnDef::new(Submission::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
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
