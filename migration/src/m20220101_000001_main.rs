use sea_orm_migration::{
    prelude::*,
    schema::{
        double, double_null, integer, integer_null, string, string_null, string_uniq,
        timestamp_with_time_zone, timestamp_with_time_zone_null, uuid, uuid_uniq,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the BNA Region table.
        manager
            .create_table(
                Table::create()
                    .table(BNARegion::Table)
                    .if_not_exists()
                    .col(string(BNARegion::Name).primary_key())
                    .to_owned(),
            )
            .await?;
        let insert_bna_regions = Query::insert()
            .into_table(BNARegion::Table)
            .columns([BNARegion::Name])
            .values_panic(["Mid-Atlantic".into()])
            .values_panic(["Midwest".into()])
            .values_panic(["Mountain".into()])
            .values_panic(["New England".into()])
            .values_panic(["Pacific".into()])
            .values_panic(["South".into()])
            .to_owned();
        manager.exec_stmt(insert_bna_regions).await?;

        // Create the country table.
        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(string(Country::Name).primary_key())
                    .to_owned(),
            )
            .await?;
        let insert_countries = Query::insert()
            .into_table(Country::Table)
            .columns([Country::Name])
            .values_panic(["Australia".into()])
            .values_panic(["Belgium".into()])
            .values_panic(["Brazil".into()])
            .values_panic(["Canada".into()])
            .values_panic(["Chile".into()])
            .values_panic(["Colombia".into()])
            .values_panic(["Croatia".into()])
            .values_panic(["Cuba".into()])
            .values_panic(["England".into()])
            .values_panic(["France".into()])
            .values_panic(["Germany".into()])
            .values_panic(["Greece".into()])
            .values_panic(["Guatemala".into()])
            .values_panic(["Iran".into()])
            .values_panic(["Iraq".into()])
            .values_panic(["Ireland".into()])
            .values_panic(["Italy".into()])
            .values_panic(["Mexico".into()])
            .values_panic(["Netherlands".into()])
            .values_panic(["New Zealand".into()])
            .values_panic(["Northern Ireland".into()])
            .values_panic(["Portugal".into()])
            .values_panic(["Scotland".into()])
            .values_panic(["Spain".into()])
            .values_panic(["United States".into()])
            .values_panic(["Vietnam".into()])
            .values_panic(["Wales".into()])
            .to_owned();
        manager.exec_stmt(insert_countries).await?;

        // Create the US state table.
        manager
            .create_table(
                Table::create()
                    .table(USState::Table)
                    .if_not_exists()
                    .col(string(USState::Name).primary_key())
                    .col(string_uniq(USState::Abbrev))
                    .col(string_uniq(USState::FIPSCode))
                    .col(integer(USState::SpeedLimit))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(USState::Table)
                    .col(USState::Abbrev)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(USState::Table)
                    .col(USState::FIPSCode)
                    .to_owned(),
            )
            .await?;
        let insert_us_states = Query::insert()
            .into_table(USState::Table)
            .columns([
                USState::Name,
                USState::Abbrev,
                USState::FIPSCode,
                USState::SpeedLimit,
            ])
            .values_panic(["Alabama".into(), "AL".into(), "01".into(), 25.into()])
            .values_panic(["Alaska".into(), "AK".into(), "02".into(), 25.into()])
            .values_panic(["Arizona".into(), "AZ".into(), "04".into(), 25.into()])
            .values_panic(["Arkansas".into(), "AR".into(), "05".into(), 30.into()])
            .values_panic(["California".into(), "CA".into(), "06".into(), 25.into()])
            .values_panic(["Colorado".into(), "CO".into(), "08".into(), 30.into()])
            .values_panic(["Connecticut".into(), "CT".into(), "09".into(), 25.into()])
            .values_panic(["Delaware".into(), "DE".into(), "10".into(), 25.into()])
            .values_panic([
                "District of Columbia".into(),
                "DC".into(),
                "11".into(),
                20.into(),
            ])
            .values_panic(["Florida".into(), "FL".into(), "12".into(), 30.into()])
            .values_panic(["Georgia".into(), "GA".into(), "13".into(), 30.into()])
            .values_panic(["Hawaii".into(), "HI".into(), "15".into(), 25.into()])
            .values_panic(["Idaho".into(), "ID".into(), "16".into(), 35.into()])
            .values_panic(["Illinois".into(), "IL".into(), "17".into(), 30.into()])
            .values_panic(["Indiana".into(), "IN".into(), "18".into(), 30.into()])
            .values_panic(["Iowa".into(), "IA".into(), "19".into(), 25.into()])
            .values_panic(["Kansas".into(), "KS".into(), "20".into(), 30.into()])
            .values_panic(["Kentucky".into(), "KY".into(), "21".into(), 35.into()])
            .values_panic(["Louisiana".into(), "LA".into(), "22".into(), 25.into()])
            .values_panic(["Maine".into(), "ME".into(), "23".into(), 25.into()])
            .values_panic(["Maryland".into(), "MD".into(), "24".into(), 30.into()])
            .values_panic(["Massachusetts".into(), "MA".into(), "25".into(), 25.into()])
            .values_panic(["Michigan".into(), "MI".into(), "26".into(), 25.into()])
            .values_panic(["Minnesota".into(), "MN".into(), "27".into(), 30.into()])
            .values_panic(["Mississippi".into(), "MS".into(), "28".into(), 25.into()])
            .values_panic(["Missouri".into(), "MO".into(), "29".into(), 25.into()])
            .values_panic(["Montana".into(), "MT".into(), "30".into(), 25.into()])
            .values_panic(["Nebraska".into(), "NE".into(), "31".into(), 25.into()])
            .values_panic(["Nevada".into(), "NV".into(), "32".into(), 25.into()])
            .values_panic(["New Hampshire".into(), "NH".into(), "33".into(), 30.into()])
            .values_panic(["New Jersey".into(), "NJ".into(), "34".into(), 25.into()])
            .values_panic(["New Mexico".into(), "NM".into(), "35".into(), 30.into()])
            .values_panic(["New York".into(), "NY".into(), "36".into(), 20.into()])
            .values_panic(["North Carolina".into(), "NC".into(), "37".into(), 35.into()])
            .values_panic(["North Dakota".into(), "ND".into(), "38".into(), 25.into()])
            .values_panic(["Ohio".into(), "OH".into(), "39".into(), 25.into()])
            .values_panic(["Oklahoma".into(), "OK".into(), "40".into(), 25.into()])
            .values_panic(["Oregon".into(), "OR".into(), "41".into(), 25.into()])
            .values_panic(["Pennsylvania".into(), "PA".into(), "42".into(), 25.into()])
            .values_panic(["Rhode Island".into(), "RI".into(), "44".into(), 25.into()])
            .values_panic(["South Carolina".into(), "SC".into(), "45".into(), 30.into()])
            .values_panic(["South Dakota".into(), "SD".into(), "46".into(), 25.into()])
            .values_panic(["Tennessee".into(), "TN".into(), "47".into(), 25.into()])
            .values_panic(["Texas".into(), "TX".into(), "48".into(), 30.into()])
            .values_panic(["Utah".into(), "UT".into(), "49".into(), 25.into()])
            .values_panic(["Vermont".into(), "VT".into(), "50".into(), 25.into()])
            .values_panic(["Virginia".into(), "VA".into(), "51".into(), 25.into()])
            .values_panic(["Washington".into(), "WA".into(), "53".into(), 25.into()])
            .values_panic(["West Virginia".into(), "WV".into(), "54".into(), 25.into()])
            .values_panic(["Wisconsin".into(), "WI".into(), "55".into(), 25.into()])
            .values_panic(["Wyoming".into(), "WY".into(), "56".into(), 30.into()])
            .values_panic(["Puerto Rico".into(), "PR".into(), "77".into(), 25.into()])
            .to_owned();
        manager.exec_stmt(insert_us_states).await?;

        // Create the StateRegionCrosswalk table.
        manager
            .create_table(
                Table::create()
                    .table(StateRegionCrosswalk::Table)
                    .col(string(StateRegionCrosswalk::State))
                    .col(string(StateRegionCrosswalk::Region))
                    .primary_key(
                        Index::create()
                            .col(StateRegionCrosswalk::State)
                            .col(StateRegionCrosswalk::Region),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StateRegionCrosswalk::Table, StateRegionCrosswalk::State)
                            .to(USState::Table, USState::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StateRegionCrosswalk::Table, StateRegionCrosswalk::Region)
                            .to(BNARegion::Table, BNARegion::Name),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(StateRegionCrosswalk::Table)
                    .col(StateRegionCrosswalk::State)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(StateRegionCrosswalk::Table)
                    .col(StateRegionCrosswalk::Region)
                    .to_owned(),
            )
            .await?;
        let insert_state_region = Query::insert()
            .into_table(StateRegionCrosswalk::Table)
            .columns([StateRegionCrosswalk::State, StateRegionCrosswalk::Region])
            .values_panic(["Alabama".into(), "South".into()])
            .values_panic(["Alaska".into(), "Pacific".into()])
            .values_panic(["Arizona".into(), "Mountain".into()])
            .values_panic(["Arkansas".into(), "South".into()])
            .values_panic(["California".into(), "Pacific".into()])
            .values_panic(["Colorado".into(), "Mountain".into()])
            .values_panic(["Connecticut".into(), "New England".into()])
            .values_panic(["Delaware".into(), "Mid-Atlantic".into()])
            .values_panic(["District of Columbia".into(), "Mid-Atlantic".into()])
            .values_panic(["Florida".into(), "South".into()])
            .values_panic(["Georgia".into(), "South".into()])
            .values_panic(["Hawaii".into(), "Pacific".into()])
            .values_panic(["Idaho".into(), "Mountain".into()])
            .values_panic(["Illinois".into(), "Midwest".into()])
            .values_panic(["Indiana".into(), "Midwest".into()])
            .values_panic(["Iowa".into(), "Midwest".into()])
            .values_panic(["Kansas".into(), "Midwest".into()])
            .values_panic(["Kentucky".into(), "South".into()])
            .values_panic(["Louisiana".into(), "South".into()])
            .values_panic(["Maine".into(), "New England".into()])
            .values_panic(["Maryland".into(), "Mid-Atlantic".into()])
            .values_panic(["Massachusetts".into(), "New England".into()])
            .values_panic(["Michigan".into(), "Midwest".into()])
            .values_panic(["Minnesota".into(), "Midwest".into()])
            .values_panic(["Mississippi".into(), "South".into()])
            .values_panic(["Missouri".into(), "Midwest".into()])
            .values_panic(["Montana".into(), "Mountain".into()])
            .values_panic(["Nebraska".into(), "Midwest".into()])
            .values_panic(["Nevada".into(), "Mountain".into()])
            .values_panic(["New Hampshire".into(), "New England".into()])
            .values_panic(["New Jersey".into(), "Mid-Atlantic".into()])
            .values_panic(["New Mexico".into(), "Mountain".into()])
            .values_panic(["New York".into(), "Mid-Atlantic".into()])
            .values_panic(["North Carolina".into(), "South".into()])
            .values_panic(["North Dakota".into(), "Midwest".into()])
            .values_panic(["Ohio".into(), "Midwest".into()])
            .values_panic(["Oklahoma".into(), "South".into()])
            .values_panic(["Oregon".into(), "Pacific".into()])
            .values_panic(["Pennsylvania".into(), "Mid-Atlantic".into()])
            .values_panic(["Rhode Island".into(), "New England".into()])
            .values_panic(["South Carolina".into(), "South".into()])
            .values_panic(["South Dakota".into(), "Midwest".into()])
            .values_panic(["Tennessee".into(), "South".into()])
            .values_panic(["Texas".into(), "South".into()])
            .values_panic(["Utah".into(), "Mountain".into()])
            .values_panic(["Vermont".into(), "New England".into()])
            .values_panic(["Virginia".into(), "South".into()])
            .values_panic(["Washington".into(), "Pacific".into()])
            .values_panic(["West Virginia".into(), "South".into()])
            .values_panic(["Wisconsin".into(), "Midwest".into()])
            .values_panic(["Wyoming".into(), "Mountain".into()])
            .values_panic(["Puerto Rico".into(), "South".into()])
            .to_owned();
        manager.exec_stmt(insert_state_region).await?;

        // Create the city table.
        manager
            .create_table(
                Table::create()
                    .table(City::Table)
                    .if_not_exists()
                    .col(uuid_uniq(City::Id))
                    .col(string(City::Country))
                    .col(string(City::State))
                    .col(string(City::Name))
                    .col(double_null(City::Latitude))
                    .col(double_null(City::Longitude))
                    .col(string_null(City::Region))
                    .col(string_null(City::StateAbbrev))
                    .col(integer_null(City::ResidentialSpeedLimit))
                    .col(
                        timestamp_with_time_zone(City::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(City::UpdatedAt))
                    .col(string_null(City::FIPSCode))
                    .primary_key(
                        Index::create()
                            .col(City::Country)
                            .col(City::State)
                            .col(City::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(City::Table, City::Country)
                            .to(Country::Table, Country::Name),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(Index::create().table(City::Table).col(City::Id).to_owned())
            .await?;

        // Create the Summary table.
        manager
            .create_table(
                Table::create()
                    .table(Summary::Table)
                    .col(uuid(Summary::Id))
                    .col(uuid(Summary::CityId))
                    .col(
                        timestamp_with_time_zone(Summary::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(Summary::PopSize))
                    .col(integer(Summary::Population))
                    .col(integer_null(Summary::ResidentialSpeedLimitOverride))
                    .col(double(Summary::Score))
                    .col(string(Summary::Version))
                    .primary_key(Index::create().col(Summary::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Summary::Table, Summary::CityId)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Summary::Table)
                    .col(Summary::CityId)
                    .col((Summary::CreatedAt, IndexOrder::Desc))
                    .col((Summary::Version, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        // Create the People table.
        manager
            .create_table(
                Table::create()
                    .table(People::Table)
                    .col(uuid(People::Id).primary_key())
                    .col(double_null(People::Score))
                    .foreign_key(
                        ForeignKey::create()
                            .from(People::Table, People::Id)
                            .to(Summary::Table, Summary::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the Retail table.
        manager
            .create_table(
                Table::create()
                    .table(Retail::Table)
                    .col(uuid(Retail::Id).primary_key())
                    .col(double_null(Retail::Score))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Retail::Table, Retail::Id)
                            .to(Summary::Table, Summary::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the Transit table.
        manager
            .create_table(
                Table::create()
                    .table(Transit::Table)
                    .col(uuid(Transit::Id).primary_key())
                    .col(double_null(Transit::Score))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Transit::Table, Transit::Id)
                            .to(Summary::Table, Summary::Id)
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
                    .col(uuid(CoreServices::Id))
                    .col(double_null(CoreServices::Dentists))
                    .col(double_null(CoreServices::Doctors))
                    .col(double_null(CoreServices::Grocery))
                    .col(double_null(CoreServices::Hospitals))
                    .col(double_null(CoreServices::Pharmacies))
                    .col(double_null(CoreServices::Score))
                    .col(double_null(CoreServices::SocialServices))
                    .primary_key(Index::create().col(CoreServices::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(CoreServices::Table, CoreServices::Id)
                            .to(Summary::Table, Summary::Id)
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
                    .col(uuid(Opportunity::Id))
                    .col(double_null(Opportunity::Employment))
                    .col(double_null(Opportunity::HigherEducation))
                    .col(double_null(Opportunity::K12Education))
                    .col(double_null(Opportunity::Score))
                    .col(double_null(Opportunity::TechnicalVocationalCollege))
                    .primary_key(Index::create().col(Opportunity::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Opportunity::Table, Opportunity::Id)
                            .to(Summary::Table, Summary::Id)
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
                    .col(uuid(Recreation::Id))
                    .col(double_null(Recreation::CommunityCenters))
                    .col(double_null(Recreation::Parks))
                    .col(double_null(Recreation::RecreationTrails))
                    .col(double_null(Recreation::Score))
                    .primary_key(Index::create().col(Recreation::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recreation::Table, Recreation::Id)
                            .to(Summary::Table, Summary::Id)
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
                    .col(uuid(Infrastructure::Id))
                    .col(double_null(Infrastructure::LowStressMiles))
                    .col(double_null(Infrastructure::HighStressMiles))
                    .primary_key(Index::create().col(Infrastructure::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Infrastructure::Table, Infrastructure::Id)
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
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Summary::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(People::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Retail::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Transit::Table).to_owned())
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
        manager
            .drop_table(Table::drop().table(USState::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(StateRegionCrosswalk::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BNARegion::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum City {
    Table,
    /// City identifier.
    Id,
    /// Country.
    Country,
    /// Creation date.
    CreatedAt,
    /// Numerical city identifier given by the U.S. census.
    FIPSCode,
    /// City latitude as defined in the U.S. census.
    Latitude,
    /// City longitude as defined in the U.S. census.
    Longitude,
    /// City name, as commonly known.
    Name,
    /// Assigned region.
    Region,
    /// City speed limit (if different from the state speed limit).
    ResidentialSpeedLimit,
    /// State name.
    State,
    /// Two-letter state abbreviation.
    StateAbbrev,
    /// Update date.
    UpdatedAt,
}

#[derive(Iden)]
pub(crate) enum Summary {
    Table,
    /// Analysis unique identifier.
    Id,
    /// City identifier.
    CityId,
    /// Creation date.
    CreatedAt,
    /// City population size category (small, medium, large).
    PopSize,
    /// City population based on the annual U.S. Census American Community Survey.
    Population,
    /// Residential speed limit override.
    ResidentialSpeedLimitOverride,
    /// BNA total score.
    Score,
    /// Analysis version in Calver format (YY.0M.[Micro])
    Version,
}

#[derive(Iden)]
enum People {
    Table,
    /// Analysis unique identifier.
    Id,
    /// BNA category score for access to residential areas.
    Score,
}

#[derive(Iden)]
enum Retail {
    Table,
    /// Analysis unique identifier.
    Id,
    /// BNA category score for access to major retail centers.
    Score,
}

#[derive(Iden)]
enum Transit {
    Table,
    /// Analysis unique identifier.
    Id,
    /// BNA category score for access to major transit stops.
    Score,
}

#[derive(Iden)]
enum CoreServices {
    Table,
    /// Analysis unique identifier.
    Id,
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
    Id,
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
    Id,
    /// BNA category subscore for access to community centers.
    CommunityCenters,
    /// BNA category subscore for access to parks.
    Parks,
    /// BNA category subscore for access to bikeable trails.
    #[allow(clippy::enum_variant_names)]
    RecreationTrails,
    /// BNA category score for access to recreational facilities.
    Score,
}

#[derive(Iden)]
enum Infrastructure {
    Table,
    /// Analysis unique identifier.
    Id,
    /// Total miles of low-stress streets and paths in the measured area.
    LowStressMiles,
    /// Total miles of high-stress streets in the measured area.
    HighStressMiles,
}

// Lookup table for the BNA regions.
#[derive(Iden)]
pub enum BNARegion {
    Table,
    /// Name of the BNA region.
    Name,
}

/// Lookup table for the state region crosswalks.
#[derive(Iden)]
pub enum StateRegionCrosswalk {
    Table,
    /// State name.
    State,
    /// BNA Region.
    Region,
}

/// Lookup table for the countries.
#[derive(Iden)]
pub enum Country {
    Table,
    /// Country name.
    Name,
}

/// Lookup table for the US states.
#[derive(Iden)]
pub enum USState {
    Table,
    /// State name.
    Name,
    /// Two-letter state abbreviation..
    Abbrev,
    /// State FIPS code.
    FIPSCode,
    /// State speed limit in mph.
    SpeedLimit,
}
