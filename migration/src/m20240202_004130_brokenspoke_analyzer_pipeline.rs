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
                    .as_enum(BrokenspokeStep::Table)
                    .values(BrokenspokeStep::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // Create the Brokenspoke Pipeline table.
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
                        ColumnDef::new(BrokenspokePipeline::Step)
                            .enumeration(BrokenspokeStep::Table, BrokenspokeStep::iter().skip(1)),
                    )
                    .col(ColumnDef::new(BrokenspokePipeline::SqsMessage).json())
                    .col(ColumnDef::new(BrokenspokePipeline::FargateTaskARN).string())
                    .col(ColumnDef::new(BrokenspokePipeline::S3Bucket).string())
                    .col(
                        ColumnDef::new(BrokenspokePipeline::StartTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(BrokenspokePipeline::EndTime).timestamp_with_time_zone())
                    .col(ColumnDef::new(BrokenspokePipeline::TornDown).boolean())
                    .col(ColumnDef::new(BrokenspokePipeline::ResultsPosted).boolean())
                    .col(ColumnDef::new(BrokenspokePipeline::Cost).decimal())
                    .to_owned(),
            )
            .await?;

        // Create the Fargate Price table.
        manager
            .create_table(
                Table::create()
                    .table(FargatePrice::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FargatePrice::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FargatePrice::PerSecond).not_null().decimal())
                    .col(
                        ColumnDef::new(FargatePrice::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BrokenspokePipeline::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FargatePrice::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum BrokenspokePipeline {
    Table,
    Cost,
    EndTime,
    FargateTaskARN,
    ResultsPosted,
    S3Bucket,
    SqsMessage,
    StartTime,
    StateMachineId,
    Step,
    TornDown,
}

#[derive(Iden, EnumIter)]
pub enum BrokenspokeStatus {
    Table,
    Pending,
    Started,
    Complete,
}

#[derive(Iden, EnumIter)]
pub enum BrokenspokeStep {
    Table,
    SqsMessage,
    Setup,
    Analysis,
    Cleanup,
}
// Pricing is coming from the CloudTempo calculator with the following paramaters:
// - Architecture: x86
// - Region: US West (Oregon)
// - Duration: 1h
// - vCPU: 2
// - Memory: 8GB
// - Storage: 200GB
//
// https://cloudtempo.dev/fargate-pricing-calculator
//
// The final value is obtained by converting the price per hour to price per second.
// This is done by dividing the price per hour per 60 and rounding it up to the
// ten thousandths (4th decimal).
//
// For instance:
// Price per hour is $0.137, which gives a price per second of 0.137/60 = 0.00228333333333.
// Rounding it up to the nearest $0.0001 brings a price of $0.0023 per second.
// https://www.calculator.net/rounding-calculator.html?cnum=0.00228333333333&cpre=4&cpren=2&cmode=nearest&sp=0&x=Calculate
#[derive(Iden)]
pub enum FargatePrice {
    Table,
    Id,
    PerSecond,
    CreatedAt,
}
