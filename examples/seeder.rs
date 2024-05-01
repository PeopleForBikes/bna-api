/// This example populates the database the database using the city ratings sample file.
use bnacore::{
    scorecard::{scorecard24::ScoreCard24, Scorecard},
    versioning::Calver,
};
use color_eyre::{eyre::Report, Result};
use csv::Reader;
use dotenv::dotenv;
use entity::{
    census, city, core_services, features, infrastructure, opportunity, prelude::*, recreation,
    speed_limit, state_region_crosswalk, state_speed_limit, summary,
};
use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue, Database, EntityTrait};
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

const US_STATE_COUNT: usize = 52;
#[derive(Debug, Deserialize)]
pub struct StateSpeedLimitCSV {
    state: String,
    fips_code_state: String,
    speed: u32,
}

#[derive(Debug, Deserialize)]
pub struct CitySpeedLimitCSV {
    fips_code_city: u32,
    speed: u32,
}

#[derive(Debug, Deserialize)]
pub struct StateRegionCrosswalkCSV {
    state_full: String,
    region: String,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Populate entities.
    let mut cities: HashMap<Uuid, city::ActiveModel> = HashMap::new();
    let mut census_populations: Vec<census::ActiveModel> = Vec::new();
    let mut speed_limits: Vec<speed_limit::ActiveModel> = Vec::new();
    let mut summaries: Vec<summary::ActiveModel> = Vec::new();
    let mut bna_features: Vec<features::ActiveModel> = Vec::new();
    let mut bna_core_services: Vec<core_services::ActiveModel> = Vec::new();
    let mut bna_recreation: Vec<recreation::ActiveModel> = Vec::new();
    let mut bna_opportunity: Vec<opportunity::ActiveModel> = Vec::new();
    let mut bna_infrastructure: Vec<infrastructure::ActiveModel> = Vec::new();
    let mut versions: HashMap<Uuid, Calver> = HashMap::new();
    let mut state_speed_limits: Vec<state_speed_limit::ActiveModel> =
        Vec::with_capacity(US_STATE_COUNT);
    let mut state_region_crosswalks: Vec<state_region_crosswalk::ActiveModel> =
        Vec::with_capacity(US_STATE_COUNT);
    let mut city_fips2limit: HashMap<u32, u32> = HashMap::new();
    let mut state_bnaregion: HashMap<String, String> = HashMap::new();

    // Load the state speed limit file.
    let mut state_speed_limit_csv_reader =
        Reader::from_path("examples/seeder-state_fips_speed.csv")?;
    for record in state_speed_limit_csv_reader.deserialize() {
        // Read the record.
        let speed_limit: StateSpeedLimitCSV = record?;

        // Populate the model.
        let state_speed_limit_model = state_speed_limit::ActiveModel {
            state_abbrev: ActiveValue::Set(speed_limit.state),
            state_fips_code: ActiveValue::Set(speed_limit.fips_code_state),
            speed: ActiveValue::Set(speed_limit.speed.try_into()?),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        state_speed_limits.push(state_speed_limit_model);
    }

    // Load the city speed limit file.
    let mut city_speed_limit_csv_reader =
        Reader::from_path("examples/seeder-city_fips_speed-v24.03.csv")?;
    for record in city_speed_limit_csv_reader.deserialize() {
        // Read the record.
        let speed_limit: CitySpeedLimitCSV = record?;
        city_fips2limit.insert(speed_limit.fips_code_city, speed_limit.speed);
    }

    // Load the state/region crosswalk CSV file.
    let mut state_region_crosswalk_csv_reader =
        Reader::from_path("examples/seeder-state_region_crosswalk-v24.05.csv")?;
    for record in state_region_crosswalk_csv_reader.deserialize() {
        // Read the record.
        let state_region: StateRegionCrosswalkCSV = record?;
        state_bnaregion.insert(state_region.state_full.clone(), state_region.region.clone());

        // Populate the state region crosswalk model.
        let r = entity::wrappers::BnaRegion::from_str(&state_region.region).unwrap();
        let state_region_crosswalk_model = state_region_crosswalk::ActiveModel {
            state: ActiveValue::Set(state_region.state_full),
            region: ActiveValue::Set(r.into()),
        };
        state_region_crosswalks.push(state_region_crosswalk_model);
    }

    // Load the historical data CSV file.
    let mut csv_reader = Reader::from_path("../../PeopleForBikes/brokenspoke/assets/city-ratings/city-ratings-all-historical-results-v24.2.csv")?;
    for record in csv_reader.deserialize() {
        // Read the record.
        let scorecard: ScoreCard24 = record?;

        // Get the record's year.
        let calver = scorecard.version();
        let version = Calver::try_from_ubuntu(&calver).unwrap();

        // Get the records creation date.
        let created_at = scorecard.creation_date;

        // Get the City UUID.
        let city_uuid = Uuid::parse_str(&scorecard.bna_id).unwrap();

        // Look for a newer summary entry than this one.
        let mut has_newer = false;
        if versions.contains_key(&city_uuid) {
            let v = versions.get(&city_uuid).unwrap();
            if version > *v {
                versions.insert(city_uuid, version.clone());
                has_newer = true;
            }
        } else {
            versions.insert(city_uuid, version.clone());
        }

        // Populate the city model.
        if !has_newer {
            let city_speed_limit = match scorecard.census_fips_code {
                Some(fips) => city_fips2limit.get(&fips).map(|x| *x as i32),
                None => None,
            };
            let bna_region = state_bnaregion
                .get(&scorecard.state_full)
                .map(|s| s.to_owned())
                .unwrap_or(scorecard.country.clone());

            let city_model = city::ActiveModel {
                city_id: ActiveValue::Set(city_uuid),
                country: ActiveValue::Set(scorecard.country.clone()),
                latitude: ActiveValue::Set(Some(scorecard.census_latitude)),
                longitude: ActiveValue::Set(Some(scorecard.census_longitude)),
                name: ActiveValue::Set(scorecard.city.clone()),
                region: ActiveValue::Set(Some(bna_region)),
                state: ActiveValue::Set(scorecard.state_full),
                state_abbrev: ActiveValue::Set(scorecard.state),
                speed_limit: ActiveValue::Set(city_speed_limit),
                created_at: ActiveValue::Set(created_at),
                updated_at: ActiveValue::NotSet,
            };
            cities.insert(city_uuid, city_model);
        }

        // Populate the census population model.
        let census_model = census::ActiveModel {
            census_id: ActiveValue::NotSet,
            city_id: ActiveValue::Set(city_uuid),
            created_at: ActiveValue::Set(created_at),
            fips_code: scorecard
                .census_fips_code
                .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(v.to_string())),
            pop_size: match scorecard.pop_size {
                bnacore::scorecard::Size::Small => ActiveValue::Set(0),
                bnacore::scorecard::Size::Medium => ActiveValue::Set(1),
                bnacore::scorecard::Size::Large => ActiveValue::Set(2),
            },
            population: ActiveValue::Set(scorecard.census_population.try_into().unwrap()),
        };
        census_populations.push(census_model);

        // Populate the speed limit model.
        let speed_limit_model = speed_limit::ActiveModel {
            speed_limit_id: ActiveValue::NotSet,
            city_id: ActiveValue::Set(city_uuid),
            created_at: ActiveValue::Set(created_at),
            residential: scorecard
                .residential_speed_limit
                .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(v.into())),
        };
        speed_limits.push(speed_limit_model);

        // Populate the summary model.
        let bna_uuid = Uuid::parse_str(scorecard.bna_uuid.as_str()).unwrap();
        let summary_model = summary::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid),
            city_id: ActiveValue::Set(city_uuid),
            created_at: ActiveValue::Set(created_at),
            score: ActiveValue::Set(scorecard.bna_rounded_score.into()),
            version: ActiveValue::Set(version.to_ubuntu()),
        };
        summaries.push(summary_model);

        // Populate the features model.
        let feature_model = features::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid),
            people: ActiveValue::Set(scorecard.bna_people),
            transit: ActiveValue::Set(scorecard.bna_transit),
            retail: ActiveValue::Set(scorecard.bna_retail),
        };
        bna_features.push(feature_model);

        // Populate the Core Services model.
        let core_services_model = core_services::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid),
            dentists: ActiveValue::Set(scorecard.bna_core_services_dentists),
            doctors: ActiveValue::Set(scorecard.bna_core_services_doctors),
            grocery: ActiveValue::Set(scorecard.bna_core_services_grocery),
            hospitals: ActiveValue::Set(scorecard.bna_core_services_hospitals),
            pharmacies: ActiveValue::Set(scorecard.bna_core_services_pharmacies),
            score: ActiveValue::Set(scorecard.bna_core_services),
            social_services: ActiveValue::Set(scorecard.bna_core_services_social_services),
        };
        bna_core_services.push(core_services_model);

        // Populate the recreation model.
        let recreation_model = recreation::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            community_centers: ActiveValue::Set(scorecard.bna_recreation_community_centers),
            parks: ActiveValue::Set(scorecard.bna_recreation_parks),
            recreation_trails: ActiveValue::Set(scorecard.bna_recreation_trails),
            score: ActiveValue::Set(scorecard.bna_recreation),
        };
        bna_recreation.push(recreation_model);

        // Populate the opportunity model.
        let opportunity_model = opportunity::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid),
            employment: ActiveValue::Set(scorecard.bna_opportunity_employment),
            higher_education: ActiveValue::Set(scorecard.bna_opportunity_higher_education),
            k12_education: ActiveValue::Set(scorecard.bna_opportunity_k12_education),
            score: ActiveValue::Set(scorecard.bna_opportunity),
            technical_vocational_college: ActiveValue::Set(
                scorecard.bna_opportunity_technical_vocational_college,
            ),
        };
        bna_opportunity.push(opportunity_model);

        // Populate the infrastructure model.
        let infratructure_model = infrastructure::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid),
            low_stress_miles: ActiveValue::Set(scorecard.bna_total_low_stress_miles),
            high_stress_miles: ActiveValue::Set(scorecard.bna_total_high_stress_miles),
        };
        bna_infrastructure.push(infratructure_model);
    }

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    // Insert the entries.
    StateSpeedLimit::insert_many(state_speed_limits)
        .exec(&db)
        .await?;

    // FIXME: Insert cities one by one to debug the cities not matching the uniqueness
    // constraint on the triplet (country,state,name).
    // Arlington, VA
    let c = cities.clone();
    for city in c.into_values() {
        let r = city.clone().insert(&db).await;
        if r.is_err() {
            dbg!(&city);
        }
    }
    // City::insert_many(cities.into_values()).exec(&db).await?;
    Census::insert_many(census_populations).exec(&db).await?;
    SpeedLimit::insert_many(speed_limits).exec(&db).await?;
    Summary::insert_many(summaries).exec(&db).await?;
    Features::insert_many(bna_features).exec(&db).await?;
    CoreServices::insert_many(
        bna_core_services
            .clone()
            .into_iter()
            .take(6000)
            .collect::<Vec<core_services::ActiveModel>>(),
    )
    .exec(&db)
    .await?;
    CoreServices::insert_many(
        bna_core_services
            .clone()
            .into_iter()
            .skip(6000)
            .collect::<Vec<core_services::ActiveModel>>(),
    )
    .exec(&db)
    .await?;
    dbg!(bna_core_services.len());
    Recreation::insert_many(bna_recreation).exec(&db).await?;
    Opportunity::insert_many(bna_opportunity).exec(&db).await?;
    Infrastructure::insert_many(bna_infrastructure)
        .exec(&db)
        .await?;

    Ok(())
}
