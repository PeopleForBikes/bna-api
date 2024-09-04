use sea_orm_migration::{
    prelude::*,
    schema::{
        boolean_null, decimal, decimal_null, integer_null, json_null, pk_auto, string, string_null,
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
                    .table(BNAPipelineStatus::Table)
                    .if_not_exists()
                    .col(string(BNAPipelineStatus::Status).primary_key())
                    .to_owned(),
            )
            .await?;
        let insert_statuses = Query::insert()
            .into_table(BNAPipelineStatus::Table)
            .columns([BNAPipelineStatus::Status])
            .values_panic(["Completed".into()])
            .values_panic(["Pending".into()])
            .values_panic(["Processing".into()])
            .to_owned();
        manager.exec_stmt(insert_statuses).await?;

        // Create the Brokenspoke Step lookup table.
        manager
            .create_table(
                Table::create()
                    .table(BNAPipelineStep::Table)
                    .if_not_exists()
                    .col(string(BNAPipelineStep::Step).primary_key())
                    .to_owned(),
            )
            .await?;
        let insert_statuses = Query::insert()
            .into_table(BNAPipelineStep::Table)
            .columns([BNAPipelineStep::Step])
            .values_panic(["Analysis".into()])
            .values_panic(["Cleanup".into()])
            .values_panic(["Save".into()])
            .values_panic(["Setup".into()])
            .to_owned();
        manager.exec_stmt(insert_statuses).await?;

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

        // Create the Brokenspoke Pipeline table.
        manager
            .create_table(
                Table::create()
                    .table(BNAPipeline::Table)
                    .if_not_exists()
                    .col(uuid(BNAPipeline::StateMachineId).primary_key())
                    .col(string_null(BNAPipeline::Step))
                    .col(json_null(BNAPipeline::SqsMessage))
                    .col(integer_null(BNAPipeline::FargatePriceId))
                    .col(string_null(BNAPipeline::FargateTaskARN))
                    .col(string_null(BNAPipeline::S3Bucket))
                    .col(string(BNAPipeline::Status).default("Pending".to_string()))
                    .col(timestamp_with_time_zone(BNAPipeline::StartTime))
                    .col(timestamp_with_time_zone_null(BNAPipeline::EndTime))
                    .col(boolean_null(BNAPipeline::TornDown))
                    .col(boolean_null(BNAPipeline::ResultsPosted))
                    .col(decimal_null(BNAPipeline::Cost))
                    .foreign_key(
                        ForeignKey::create()
                            .from(BNAPipeline::Table, BNAPipeline::Step)
                            .to(BNAPipelineStep::Table, BNAPipelineStep::Step),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BNAPipeline::Table, BNAPipeline::Status)
                            .to(BNAPipelineStatus::Table, BNAPipelineStatus::Status),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BNAPipeline::Table, BNAPipeline::FargatePriceId)
                            .to(FargatePrice::Table, FargatePrice::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BNAPipeline::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BNAPipelineStatus::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BNAPipelineStep::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FargatePrice::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum BNAPipeline {
    Table,
    Cost,
    EndTime,
    FargatePriceId,
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
pub enum BNAPipelineStatus {
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
pub enum BNAPipelineStep {
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
