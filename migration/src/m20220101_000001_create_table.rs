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
                    .col(ColumnDef::new(City::Id).uuid().not_null().primary_key())
                    // .col(
                    //     ColumnDef::new(City::CreatedAt)
                    //         .timestamp_with_time_zone()
                    //         .not_null(),
                    // )
                    // .col(
                    //     ColumnDef::new(City::UpdatedAt)
                    //         .timestamp_with_time_zone()
                    //         .not_null(),
                    // )
                    .col(ColumnDef::new(City::CensusFipsCode).unsigned().not_null())
                    .col(ColumnDef::new(City::CensusLatitude).double().not_null())
                    .col(ColumnDef::new(City::CensusLongitude).double().not_null())
                    .col(ColumnDef::new(City::CensusPopulation).unsigned().not_null())
                    .col(ColumnDef::new(City::City).string().not_null())
                    .col(ColumnDef::new(City::Country).string().not_null())
                    .col(ColumnDef::new(City::PopSize).string().not_null())
                    .col(ColumnDef::new(City::Rank).unsigned().not_null())
                    .col(ColumnDef::new(City::RankCountry).unsigned().not_null())
                    .col(ColumnDef::new(City::RankCountrySize).unsigned().not_null())
                    .col(ColumnDef::new(City::RankSize).unsigned().not_null())
                    .col(ColumnDef::new(City::RankState).unsigned().not_null())
                    .col(ColumnDef::new(City::Region).string().not_null())
                    .col(
                        ColumnDef::new(City::ResidentialSpeedLimit)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(City::State).string().not_null())
                    .col(ColumnDef::new(City::StateFull).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create the BNA table.
        manager
            .create_table(
                Table::create()
                    .table(Bna::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bna::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    // .col(
                    //     ColumnDef::new(Bna::CreatedAt)
                    //         .timestamp_with_time_zone()
                    //         .not_null(),
                    // )
                    // .col(
                    //     ColumnDef::new(Bna::UpdatedAt)
                    //         .timestamp_with_time_zone()
                    //         .not_null(),
                    // )
                    .col(ColumnDef::new(Bna::BNACoreServices).double().not_null())
                    .col(
                        ColumnDef::new(Bna::BNACoreServicesDentists)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bna::BNACoreServicesDoctors).double())
                    .col(
                        ColumnDef::new(Bna::BNACoreServicesGrocery)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bna::BNACoreServicesHospitals).double())
                    .col(
                        ColumnDef::new(Bna::BNACoreServicesPharmacies)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bna::BNACoreServicesSocialServices)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bna::BNAOpportunity).double().not_null())
                    .col(
                        ColumnDef::new(Bna::BNAOpportunityEmployment)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bna::BNAOpportunityHigherEducation)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bna::BNAOpportunityK12Education)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bna::BNAOpportunityTechnicalVocationalCollege)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bna::BNAOverallScore).double().not_null())
                    .col(ColumnDef::new(Bna::BNAPeople).double().not_null())
                    .col(ColumnDef::new(Bna::BNARecreation).double().not_null())
                    .col(
                        ColumnDef::new(Bna::BNARecreationCommunityCenters)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bna::BNARecreationParks).double().not_null())
                    .col(ColumnDef::new(Bna::BNARecreationTrails).double().not_null())
                    .col(ColumnDef::new(Bna::BNARetail).double().not_null())
                    .col(ColumnDef::new(Bna::BNARoundedScore).unsigned().not_null())
                    .col(ColumnDef::new(Bna::BNATransit).double().not_null())
                    // .col(ColumnDef::new(Bna::BNAUuid).uuid().not_null())
                    .col(ColumnDef::new(Bna::CityId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bna::Table, Bna::CityId)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        //         // Create the community_survey table.
        //         manager
        //             .create_table(
        //                 Table::create()
        //                     .table(CommunitySurvey::Table)
        //                     .if_not_exists()
        //                     .col(
        //                         ColumnDef::new(CommunitySurvey::Id)
        //                             .uuid()
        //                             .not_null()
        //                             .primary_key(),
        //                     )
        //                     .col(ColumnDef::new(CommunitySurvey::Network).double().not_null())
        //                     .col(
        //                         ColumnDef::new(CommunitySurvey::Awareness)
        //                             .double()
        //                             .not_null(),
        //                     )
        //                     .col(ColumnDef::new(CommunitySurvey::Safety).double().not_null())
        //                     .col(
        //                         ColumnDef::new(CommunitySurvey::Ridership)
        //                             .double()
        //                             .not_null(),
        //                     )
        //                     .col(ColumnDef::new(CommunitySurvey::Total).double().not_null())
        //                     .col(
        //                         ColumnDef::new(CommunitySurvey::Responses)
        //                             .unsigned()
        //                             .not_null(),
        //                     )
        //                     .col(ColumnDef::new(CommunitySurvey::CityId).uuid())
        //                     .col(
        //                         ColumnDef::new(CommunitySurvey::CreatedAt)
        //                             .timestamp_with_time_zone()
        //                             .not_null(),
        //                     )
        //                     .col(
        //                         ColumnDef::new(CommunitySurvey::UpdatedAt)
        //                             .timestamp_with_time_zone()
        //                             .not_null(),
        //                     )
        //                     .foreign_key(
        //                         ForeignKey::create()
        //                             .from(CommunitySurvey::Table, CommunitySurvey::CityId)
        //                             .to(City::Table, City::Id)
        //                             .on_delete(ForeignKeyAction::Cascade)
        //                             .on_update(ForeignKeyAction::Cascade),
        //                     )
        //                     .to_owned(),
        //             )
        //             .await?;

        // Create the infrastructure table.
        manager
            .create_table(
                Table::create()
                    .table(Infrastructure::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Infrastructure::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Infrastructure::BNATotalLowStressMiles)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Infrastructure::BNATotalHighStressMiles)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Infrastructure::CityId).uuid())
                    // .col(
                    //     ColumnDef::new(Infrastructure::CreatedAt)
                    //         .timestamp_with_time_zone()
                    //         .not_null(),
                    // )
                    // .col(
                    //     ColumnDef::new(Infrastructure::UpdatedAt)
                    //         .timestamp_with_time_zone()
                    //         .not_null(),
                    // )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Infrastructure::Table, Infrastructure::CityId)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bna::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await?;
        // manager
        //     .drop_table(Table::drop().table(CommunitySurvey::Table).to_owned())
        //     .await?;
        manager
            .drop_table(Table::drop().table(Infrastructure::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum City {
    Table,
    Id,
    // CreatedAt,
    // UpdatedAt,
    /// Numerical city identifier given by the U.S. census.
    CensusFipsCode,
    /// City latitude as defined in the U.S. census.
    CensusLatitude,
    /// City longitude as defined in the U.S. census.
    CensusLongitude,
    /// City population based on the annual U.S. Census American Community Survey.
    CensusPopulation,
    /// City name, as commonly known.
    City,
    /// Country.
    Country,
    /// City population size category (small, medium, large).
    PopSize,
    /// BNA score rank among all cities rated.
    Rank,
    /// BNA score rank among all cities in the same country.
    RankCountry,
    /// BNA score rank among all cities of the same population size in the same country.
    RankCountrySize,
    /// BNA score rank among all cities of the same population size rated.
    RankSize,
    /// BNA score rank among all cities in the same state.
    RankState,
    /// Assigned region.
    Region,
    /// Assumed speed limit on residential roads (mph).
    ResidentialSpeedLimit,
    /// State abbreviation.
    State,
    /// State name.
    StateFull,
}

#[derive(Iden)]
enum Bna {
    Table,
    Id,
    CityId,
    // CreatedAt,
    // UpdatedAt,
    /// BNA category score for access to core services.
    BNACoreServices,
    /// BNA category subscore for access to dentists.
    BNACoreServicesDentists,
    /// BNA category subscore for access to doctors.
    BNACoreServicesDoctors,
    /// BNA category subscore for access to grocery stores.
    BNACoreServicesGrocery,
    /// BNA category subscore for access to hospitals.
    BNACoreServicesHospitals,
    /// BNA category subscore for access to pharmacies.
    BNACoreServicesPharmacies,
    /// BNA category subscore for access to social services.
    BNACoreServicesSocialServices,
    /// BNA category score for access to education and jobs.
    BNAOpportunity,
    /// BNA category subscore for access to job location areas.
    BNAOpportunityEmployment,
    /// BNA category subscore for access to universities and colleges,
    BNAOpportunityHigherEducation,
    /// BNA category subscore for access to k12 schools
    BNAOpportunityK12Education,
    /// BNA category subscore for access to technical and vocational colleges.
    BNAOpportunityTechnicalVocationalCollege,
    /// BNA total score.
    BNAOverallScore,
    /// BNA category score for access to residential areas.
    BNAPeople,
    /// BNA category score for access to recreational facilities.
    BNARecreation,
    /// BNA category subscore for access to community centers.
    BNARecreationCommunityCenters,
    /// BNA category subscore for access to parks.
    BNARecreationParks,
    /// BNA category subscore for access to bikeable trails.
    BNARecreationTrails,
    /// BNA category score for access to major retail centers.
    BNARetail,
    /// BNA total score rounded.
    BNARoundedScore,
    /// BNA category score for access to major transit stops.
    BNATransit,
}

// #[derive(Iden)]
// enum CommunitySurvey {
//     Table,
//     Id,
//     CityId,
//     CreatedAt,
//     UpdatedAt,
//     /// Community survey awareness subscore.
//     Awareness,
//     /// Community survey network subscore.
//     Network,
//     /// Number of responses to community survey.
//     Responses,
//     /// Community survey ridership subscore.
//     Ridership,
//     /// Community survey safety subscore.
//     Safety,
//     /// Community survey overall score.
//     Total,
//     /// Community survey overall score rounded to nearest whole number.
//     TotalRounded,
// }

#[derive(Iden)]
enum Infrastructure {
    Table,
    Id,
    CityId,
    // CreatedAt,
    // UpdatedAt,
    /// Total miles of low-stress streets and paths in the measured area.
    BNATotalLowStressMiles,
    /// Total miles of high-stress streets in the measured area.
    BNATotalHighStressMiles,
}
