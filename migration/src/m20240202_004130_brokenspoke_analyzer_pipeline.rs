use sea_orm_migration::{
    prelude::*,
    schema::{
        boolean_null, decimal, decimal_null, json_null, pk_auto, string, string_null,
        timestamp_with_time_zone, timestamp_with_time_zone_null, uuid,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the Brokenspoke Status lookup table.
        manager
            .create_table(
                Table::create()
                    .table(BrokenspokeStatus::Table)
                    .if_not_exists()
                    .col(string(BrokenspokeStatus::Status).primary_key())
                    .to_owned(),
            )
            .await?;

        // Create the Brokenspoke Step lookup table.
        manager
            .create_table(
                Table::create()
                    .table(BrokenspokeStep::Table)
                    .if_not_exists()
                    .col(string(BrokenspokeStep::Step).primary_key())
                    .to_owned(),
            )
            .await?;

        // Create the Brokenspoke Pipeline table.
        manager
            .create_table(
                Table::create()
                    .table(BrokenspokePipeline::Table)
                    .if_not_exists()
                    .col(uuid(BrokenspokePipeline::StateMachineId).primary_key())
                    .col(string_null(BrokenspokePipeline::Step).string())
                    .col(json_null(BrokenspokePipeline::SqsMessage).json())
                    .col(decimal_null(BrokenspokePipeline::FargatePrice).string())
                    .col(string_null(BrokenspokePipeline::FargateTaskARN).string())
                    .col(string_null(BrokenspokePipeline::S3Bucket).string())
                    .col(string(BrokenspokePipeline::Status).string())
                    .col(timestamp_with_time_zone(BrokenspokePipeline::StartTime))
                    .col(timestamp_with_time_zone_null(BrokenspokePipeline::EndTime))
                    .col(boolean_null(BrokenspokePipeline::TornDown))
                    .col(boolean_null(BrokenspokePipeline::ResultsPosted).boolean())
                    .col(decimal_null(BrokenspokePipeline::Cost).decimal())
                    .foreign_key(
                        ForeignKey::create()
                            .from(BrokenspokePipeline::Table, BrokenspokePipeline::Step)
                            .to(BrokenspokeStep::Table, BrokenspokeStep::Step),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BrokenspokePipeline::Table, BrokenspokePipeline::Status)
                            .to(BrokenspokeStatus::Table, BrokenspokeStatus::Status),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                BrokenspokePipeline::Table,
                                BrokenspokePipeline::FargatePrice,
                            )
                            .to(FargatePrice::Table, FargatePrice::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the Fargate Price table.
        manager
            .create_table(
                Table::create()
                    .table(FargatePrice::Table)
                    .if_not_exists()
                    .col(pk_auto(FargatePrice::Id))
                    .col(decimal(FargatePrice::PerSecond))
                    .col(
                        timestamp_with_time_zone(FargatePrice::CreatedAt)
                            .default(Expr::current_timestamp()),
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
            .drop_table(Table::drop().table(BrokenspokeStatus::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BrokenspokeStep::Table).to_owned())
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
    FargatePrice,
    FargateTaskARN,
    ResultsPosted,
    S3Bucket,
    SqsMessage,
    StartTime,
    StateMachineId,
    Status,
    Step,
    TornDown,
}

/// Lookup table for the brokenspoke statuses.
//
//     Pending,
//     Started,
//     Complete,
#[derive(Iden)]
pub enum BrokenspokeStatus {
    Table,
    Status,
}

/// Lookup table for the brokenspoke steps.
//
// SqsMessage,
// Setup,
// Analysis,
// Cleanup,
#[derive(Iden)]
pub enum BrokenspokeStep {
    Table,
    Step,
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
