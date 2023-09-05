use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the city table.
        manager
            .create_table(
                Table::create()
                    .table(City::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(City::CityId).uuid().not_null().primary_key())
                    .col(ColumnDef::new(City::Country).string().not_null())
                    .col(ColumnDef::new(City::Latitude).double().not_null())
                    .col(ColumnDef::new(City::Longitude).double().not_null())
                    .col(ColumnDef::new(City::Name).string().not_null())
                    .col(ColumnDef::new(City::Region).string().not_null())
                    .col(ColumnDef::new(City::State).string().not_null())
                    .col(ColumnDef::new(City::StateAbbrev).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(City::Table)
                    .col(City::CityId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(City::Table)
                    .col(City::Name)
                    .to_owned(),
            )
            .await?;

        // Create CensusPopulation table.
        manager
            .create_table(
                Table::create()
                    .table(Census::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Census::CensusId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Census::CityId).uuid().not_null())
                    .col(ColumnDef::new(Census::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Census::FIPSCode).string().not_null())
                    .col(ColumnDef::new(Census::PopSize).integer().not_null())
                    .col(ColumnDef::new(Census::Population).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Census::Table, Census::CityId)
                            .to(City::Table, City::CityId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Census::Table)
                    .col(Census::CensusId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Census::Table)
                    .col(Census::CityId)
                    .to_owned(),
            )
            .await?;

        // Create Ranking table.
        manager
            .create_table(
                Table::create()
                    .table(Ranking::Table)
                    .col(
                        ColumnDef::new(Ranking::RankingId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ranking::CityId).uuid().not_null())
                    .col(ColumnDef::new(Ranking::Country).integer().not_null())
                    .col(ColumnDef::new(Ranking::CountrySize).integer().not_null())
                    .col(ColumnDef::new(Ranking::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Ranking::Global).integer().not_null())
                    .col(ColumnDef::new(Ranking::Size).integer().not_null())
                    .col(ColumnDef::new(Ranking::State).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Ranking::Table, Ranking::CityId)
                            .to(City::Table, City::CityId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Ranking::Table)
                    .col(Ranking::RankingId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Ranking::Table)
                    .col(Ranking::CityId)
                    .to_owned(),
            )
            .await?;

        // Create the speed limit table.
        manager
            .create_table(
                Table::create()
                    .table(SpeedLimit::Table)
                    .col(
                        ColumnDef::new(SpeedLimit::SpeedLimitId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SpeedLimit::CityId).uuid().not_null())
                    .col(ColumnDef::new(SpeedLimit::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(SpeedLimit::Residential).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SpeedLimit::Table, SpeedLimit::CityId)
                            .to(City::Table, City::CityId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(SpeedLimit::Table)
                    .col(SpeedLimit::SpeedLimitId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(SpeedLimit::Table)
                    .col(SpeedLimit::CityId)
                    .to_owned(),
            )
            .await?;

        // Create the summary table.
        manager
            .create_table(
                Table::create()
                    .table(Summary::Table)
                    .col(ColumnDef::new(Summary::BNAUuid).uuid().not_null())
                    .col(ColumnDef::new(Summary::CityId).uuid().not_null())
                    .col(ColumnDef::new(Summary::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Summary::Score).double().not_null())
                    .primary_key(Index::create().col(Summary::BNAUuid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Summary::Table, Summary::CityId)
                            .to(City::Table, City::CityId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the features table.
        manager
            .create_table(
                Table::create()
                    .table(Features::Table)
                    .col(ColumnDef::new(Features::BNAUuid).uuid().not_null())
                    .col(ColumnDef::new(Features::People).double().not_null())
                    .col(ColumnDef::new(Features::Retail).double().not_null())
                    .col(ColumnDef::new(Features::Transit).double().not_null())
                    .primary_key(Index::create().col(Features::BNAUuid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Features::Table, Features::BNAUuid)
                            .to(Summary::Table, Summary::BNAUuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the CoreServices table.
        manager
            .create_table(
                Table::create()
                    .table(CoreServices::Table)
                    .col(ColumnDef::new(CoreServices::BNAUuid).uuid().not_null())
                    .col(ColumnDef::new(CoreServices::Dentists).double().not_null())
                    .col(ColumnDef::new(CoreServices::Doctors).double().not_null())
                    .col(ColumnDef::new(CoreServices::Grocery).double().not_null())
                    .col(ColumnDef::new(CoreServices::Hospitals).double().not_null())
                    .col(ColumnDef::new(CoreServices::Pharmacies).double().not_null())
                    .col(ColumnDef::new(CoreServices::Score).double().not_null())
                    .col(
                        ColumnDef::new(CoreServices::SocialServices)
                            .double()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(CoreServices::BNAUuid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(CoreServices::Table, CoreServices::BNAUuid)
                            .to(Summary::Table, Summary::BNAUuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the Opportunity table.
        manager
            .create_table(
                Table::create()
                    .table(Opportunity::Table)
                    .col(ColumnDef::new(Opportunity::BNAUuid).uuid().not_null())
                    .col(ColumnDef::new(Opportunity::Employment).double().not_null())
                    .col(
                        ColumnDef::new(Opportunity::HigherEducation)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Opportunity::K12Education)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Opportunity::Score).double().not_null())
                    .col(
                        ColumnDef::new(Opportunity::TechnicalVocationalCollege)
                            .double()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(Opportunity::BNAUuid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Opportunity::Table, Opportunity::BNAUuid)
                            .to(Summary::Table, Summary::BNAUuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the Recreation table.
        manager
            .create_table(
                Table::create()
                    .table(Recreation::Table)
                    .col(ColumnDef::new(Recreation::BNAUuid).uuid().not_null())
                    .col(
                        ColumnDef::new(Recreation::CommunityCenters)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Recreation::Parks).double().not_null())
                    .col(
                        ColumnDef::new(Recreation::RecreationTrails)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Recreation::Score).double().not_null())
                    .primary_key(Index::create().col(Recreation::BNAUuid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recreation::Table, Recreation::BNAUuid)
                            .to(Summary::Table, Summary::BNAUuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the Infrastructure table.
        manager
            .create_table(
                Table::create()
                    .table(Infrastructure::Table)
                    .col(ColumnDef::new(Infrastructure::BNAUuid).uuid().not_null())
                    .col(
                        ColumnDef::new(Infrastructure::LowStressMiles)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Infrastructure::HighStressMiles)
                            .double()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(Infrastructure::BNAUuid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Infrastructure::Table, Infrastructure::BNAUuid)
                            .to(Summary::Table, Summary::BNAUuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Census::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Ranking::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SpeedLimit::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Summary::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Features::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CoreServices::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Opportunity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Recreation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Infrastructure::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum City {
    Table,
    /// City identifier.
    CityId,
    /// Country.
    Country,
    /// City latitude as defined in the U.S. census.
    Latitude,
    /// City longitude as defined in the U.S. census.
    Longitude,
    /// City name, as commonly known.
    Name,
    /// Assigned region.
    Region,
    /// State name.
    State,
    /// Two-letter state abbreviation.
    StateAbbrev,
}

#[derive(Iden)]
enum Census {
    Table,
    CensusId,
    /// City identifier.
    CityId,
    /// Creation date.
    CreatedAt,
    /// Numerical city identifier given by the U.S. census.
    FIPSCode,
    /// City population size category (small, medium, large).
    PopSize,
    /// City population based on the annual U.S. Census American Community Survey.
    Population,
}

#[derive(Iden)]
enum Ranking {
    Table,
    RankingId,
    /// City identifier.
    CityId,
    /// Creation date.
    CreatedAt,
    /// Country ranking.
    Country,
    /// Country size.
    CountrySize,
    /// Global ranking.
    Global,
    Size,
    State,
}

#[derive(Iden)]
enum SpeedLimit {
    Table,
    SpeedLimitId,
    /// City identifier.
    CityId,
    /// Creation date.
    CreatedAt,
    /// Residential speed limit.
    Residential,
}

#[derive(Iden)]
enum Summary {
    Table,
    /// Analysis unique identifier.
    BNAUuid,
    /// City identifier.
    CityId,
    /// Creation date.
    CreatedAt,
    /// BNA total score.
    Score,
}

#[derive(Iden)]
enum Features {
    Table,
    /// Analysis unique identifier.
    BNAUuid,
    /// BNA category score for access to residential areas.
    People,
    /// BNA category score for access to major retail centers.
    Retail,
    /// BNA category score for access to major transit stops.
    Transit,
}

#[derive(Iden)]
enum CoreServices {
    Table,
    /// Analysis unique identifier.
    BNAUuid,
    /// BNA category subscore for access to dentists.
    Dentists,
    /// BNA category subscore for access to doctors.
    Doctors,
    /// BNA category subscore for access to grocery stores.
    Grocery,
    /// BNA category subscore for access to hospitals.
    Hospitals,
    /// BNA category subscore for access to pharmacies.
    Pharmacies,
    /// BNA category score for access to core services.
    Score,
    /// BNA category subscore for access to social services.
    SocialServices,
}

#[derive(Iden)]
enum Opportunity {
    Table,
    /// Analysis unique identifier.
    BNAUuid,
    /// BNA category subscore for access to job location areas.
    Employment,
    /// BNA category subscore for access to universities and colleges.
    HigherEducation,
    /// BNA category subscore for access to k12 schools
    K12Education,
    /// BNA category score for access to education and jobs.
    Score,
    /// BNA category subscore for access to technical and vocational colleges.
    TechnicalVocationalCollege,
}

#[derive(Iden)]
enum Recreation {
    Table,
    /// Analysis unique identifier.
    BNAUuid,
    /// BNA category subscore for access to community centers.
    CommunityCenters,
    /// BNA category subscore for access to parks.
    Parks,
    /// BNA category subscore for access to bikeable trails.
    RecreationTrails,
    /// BNA category score for access to recreational facilities.
    Score,
}

#[derive(Iden)]
enum Infrastructure {
    Table,
    /// Analysis unique identifier.
    BNAUuid,
    /// Total miles of low-stress streets and paths in the measured area.
    LowStressMiles,
    /// Total miles of high-stress streets in the measured area.
    HighStressMiles,
}
