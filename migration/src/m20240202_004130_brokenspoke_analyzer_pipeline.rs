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
                    .as_enum(BrokenspokeStatus::Table)
                    .values(BrokenspokeStatus::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // Create the approval status type.
        manager
            .create_type(
                Type::create()
                    .as_enum(BrokenspokeState::Table)
                    .values(BrokenspokeState::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BrokenspokePipeline::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BrokenspokePipeline::StateMachineId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BrokenspokePipeline::ScheduledTriggerId)
                            .uuid()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(BrokenspokePipeline::State)
                            .enumeration(BrokenspokeState::Table, BrokenspokeState::iter().skip(1)),
                    )
                    .col(ColumnDef::new(BrokenspokePipeline::SqsMessage).json())
                    .col(ColumnDef::new(BrokenspokePipeline::NeonBranchId).string())
                    .col(ColumnDef::new(BrokenspokePipeline::FargateTaskId).uuid())
                    .col(ColumnDef::new(BrokenspokePipeline::S3Bucket).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BrokenspokePipeline::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BrokenspokePipeline {
    Table,
    StateMachineId,
    ScheduledTriggerId,
    State,
    SqsMessage,
    NeonBranchId,
    FargateTaskId,
    S3Bucket,
}

#[derive(Iden, EnumIter)]
pub enum BrokenspokeStatus {
    Table,
    Pending,
    Started,
    Complete,
}

#[derive(Iden, EnumIter)]
pub enum BrokenspokeState {
    Table,
    Pipeline,
    SqsMessage,
    Setup,
    Analysis,
    Export,
}
