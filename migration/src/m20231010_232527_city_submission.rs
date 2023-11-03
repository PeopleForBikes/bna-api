use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .to_owned(),
            )
            .await
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
}
