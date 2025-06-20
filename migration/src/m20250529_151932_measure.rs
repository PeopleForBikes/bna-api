use crate::m20220101_000001_main::Summary;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BikeLaneType::Table)
                    .if_not_exists()
                    .col(string(BikeLaneType::Name).primary_key())
                    .to_owned(),
            )
            .await?;
        let insert_bike_lane_types = Query::insert()
            .into_table(BikeLaneType::Table)
            .columns([BikeLaneType::Name])
            .values_panic(["buffered_lane".into()])
            .values_panic(["lane".into()])
            .values_panic(["path".into()])
            .values_panic(["sharrow".into()])
            .values_panic(["track".into()])
            .to_owned();
        manager.exec_stmt(insert_bike_lane_types).await?;

        manager
            .create_table(
                Table::create()
                    .table(Measure::Table)
                    .if_not_exists()
                    .col(uuid(Measure::Id).primary_key())
                    .col(double_null(Measure::BufferedLane))
                    .col(double_null(Measure::Lane))
                    .col(double_null(Measure::Path))
                    .col(double_null(Measure::Sharrow))
                    .col(double_null(Measure::Track))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Measure::Table, Measure::Id)
                            .to(Summary::Table, Summary::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BikeLaneType::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Measure::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Lookup table for the various bike lane types.
#[derive(DeriveIden)]
enum BikeLaneType {
    Table,
    /// Name of the bike lane type.
    Name,
}

/// Store the results returned by Measure.
#[derive(DeriveIden)]
enum Measure {
    Table,
    /// Analysis unique identifier.
    Id,
    /// Miles of buffered bike lanes.
    BufferedLane,
    /// Miles of bike lanes.
    Lane,
    /// Miles of off-street paths.
    Path,
    /// Miles of sharrows.
    Sharrow,
    /// Miles of tracks.
    Track,
}
