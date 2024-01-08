use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the approval status type.
        manager
            .create_type(
                Type::create()
                    .as_enum(ApprovalStatus::Table)
                    .values(ApprovalStatus::iter().skip(1))
                    .to_owned(),
            )
            .await?;

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
                    .col(ColumnDef::new(Submission::Title).string())
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
                    .col(
                        ColumnDef::new(Submission::Status)
                            .enumeration(ApprovalStatus::Table, ApprovalStatus::iter().skip(1))
                            .default(ApprovalStatus::Pending.to_string()),
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

#[derive(DeriveIden)]
enum Submission {
    Table,
    Id,
    FirstName,
    LastName,
    Title,
    Organization,
    Email,
    Country,
    City,
    Region,
    FIPSCode,
    Consent,
    Status,
}

#[derive(Iden, EnumIter)]
pub enum ApprovalStatus {
    Table,
    #[iden = "Pending"]
    Pending,
    #[iden = "Approved"]
    Approved,
    #[iden = "Rejected"]
    Rejected,
}
