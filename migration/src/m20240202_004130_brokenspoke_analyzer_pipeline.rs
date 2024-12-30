use sea_orm_migration::{
    prelude::*,
    schema::{
        decimal, decimal_null, integer_null, json_null, pk_auto, string, string_null,
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
        let insert_prices = Query::insert()
            .into_table(FargatePrice::Table)
            .columns([FargatePrice::PerSecond])
            .values_panic([0.000038.into()])
            .to_owned();
        manager.exec_stmt(insert_prices).await?;

        // Create the Brokenspoke Pipeline table.
        manager
            .create_table(
                Table::create()
                    .table(BNAPipeline::Table)
                    .if_not_exists()
                    .col(uuid(BNAPipeline::StateMachineId).primary_key())
                    .col(string(BNAPipeline::Step))
                    .col(json_null(BNAPipeline::SqsMessage))
                    .col(integer_null(BNAPipeline::FargatePriceId))
                    .col(string_null(BNAPipeline::FargateTaskARN))
                    .col(string_null(BNAPipeline::S3Bucket))
                    .col(string(BNAPipeline::Status).default("Pending".to_string()))
                    .col(timestamp_with_time_zone(BNAPipeline::StartTime))
                    .col(timestamp_with_time_zone_null(BNAPipeline::EndTime))
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
    S3Bucket,
    SqsMessage,
    StartTime,
    StateMachineId,
    Status,
    Step,
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
// This is done by dividing the price per hour per 3600 and rounding it up to the
// ten thousandths (4th decimal).
//
// For instance:
// Price per hour is $0.137, which gives a price per second of 0.137/3600 = 0.0000380555555556.
// Rounding it up to the nearest $0.000001 brings a price of $0.000038 per second.
// Use for example https://www.calculator.io/rounding-calculator/.
#[derive(Iden)]
pub enum FargatePrice {
    Table,
    Id,
    PerSecond,
    CreatedAt,
}
