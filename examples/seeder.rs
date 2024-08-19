/// This example populates the database the database using the city ratings sample file.
use bnacore::{
    scorecard::{scorecard24::ScoreCard24, Scorecard},
    versioning::Calver,
};
use color_eyre::{eyre::Report, Result};
use csv::Reader;
use dotenv::dotenv;
use entity::{
    census, city, core_services, infrastructure, opportunity, people, prelude::*, recreation,
    retail, speed_limit, summary, transit,
};
use sea_orm::{prelude::Uuid, ActiveValue, Database, EntityTrait};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

const US_STATE_COUNT: usize = 51; // w/ Puerto Rico
const CHUNK_SIZE: usize = 1000;
const BNA_COUNTRY_COUNT: usize = 15;

#[derive(Debug, Deserialize)]
pub struct CitySpeedLimitCSV {
    fips_code_city: u32,
    speed: u32,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Populate entities.
    let mut cities: HashMap<Uuid, city::ActiveModel> = HashMap::new();
    let mut census_populations: Vec<census::ActiveModel> = Vec::new();
    let mut speed_limits: Vec<speed_limit::ActiveModel> = Vec::new();
    let mut summaries: Vec<summary::ActiveModel> = Vec::new();
    let mut bna_people: Vec<people::ActiveModel> = Vec::new();
    let mut bna_retail: Vec<retail::ActiveModel> = Vec::new();
    let mut bna_transit: Vec<transit::ActiveModel> = Vec::new();
    let mut bna_core_services: Vec<core_services::ActiveModel> = Vec::new();
    let mut bna_recreation: Vec<recreation::ActiveModel> = Vec::new();
    let mut bna_opportunity: Vec<opportunity::ActiveModel> = Vec::new();
    let mut bna_infrastructure: Vec<infrastructure::ActiveModel> = Vec::new();
    let mut versions: HashMap<Uuid, Calver> = HashMap::new();
    let mut city_fips2limit: HashMap<u32, u32> = HashMap::new();

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    // Load the US States Region Crosswalk.
    let state_region_models = StateRegionCrosswalk::find().all(&db).await?;
    let mut state_regions: HashMap<String, String> = HashMap::with_capacity(US_STATE_COUNT);
    for state_region in state_region_models {
        state_regions.insert(state_region.state, state_region.region);
    }

    // Load the available countries.
    let country_models = Country::find().all(&db).await?;
    let mut countries: HashSet<String> = HashSet::with_capacity(BNA_COUNTRY_COUNT);
    for country in country_models {
        countries.insert(country.name);
    }

    // Load the city speed limit file.
    let mut city_speed_limit_csv_reader =
        Reader::from_path("examples/seeder-city_fips_speed-v24.03.csv")?;
    for record in city_speed_limit_csv_reader.deserialize() {
        // Read the record.
        let speed_limit: CitySpeedLimitCSV = record?;
        city_fips2limit.insert(speed_limit.fips_code_city, speed_limit.speed);
    }

    // Load the historical data CSV file.
    let mut csv_reader = Reader::from_path("../../PeopleForBikes/brokenspoke/assets/city-ratings/city-ratings-all-historical-results-v24.07.csv")?;
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
            let bna_region = state_regions
                .get(&scorecard.state_full)
                .map(|s| s.to_owned())
                .unwrap_or(scorecard.country.clone());
            let err_msg = format!("cannot find country {}", scorecard.country.clone());
            let country = countries
                .get(&scorecard.country.clone())
                .expect(err_msg.as_str())
                .to_string();

            let city_model = city::ActiveModel {
                id: ActiveValue::Set(city_uuid),
                country: ActiveValue::Set(country),
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
            id: ActiveValue::NotSet,
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
            id: ActiveValue::NotSet,
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
            bna_id: ActiveValue::Set(bna_uuid),
            city_id: ActiveValue::Set(city_uuid),
            created_at: ActiveValue::Set(created_at),
            score: ActiveValue::Set(scorecard.bna_rounded_score.into()),
            version: ActiveValue::Set(version.to_ubuntu()),
        };
        summaries.push(summary_model);

        // Populate the People model.
        let feature_model = people::ActiveModel {
            bna_id: ActiveValue::Set(bna_uuid),
            people: ActiveValue::Set(scorecard.bna_people),
        };
        bna_people.push(feature_model);

        // Populate the Retail model.
        let retail_model = retail::ActiveModel {
            bna_id: ActiveValue::Set(bna_uuid),
            retail: ActiveValue::Set(scorecard.bna_retail),
        };
        bna_retail.push(retail_model);

        // Populate the Transit model.
        let transit_model = transit::ActiveModel {
            bna_id: ActiveValue::Set(bna_uuid),
            transit: ActiveValue::Set(scorecard.bna_transit),
        };
        bna_transit.push(transit_model);

        // Populate the Core Services model.
        let core_services_model = core_services::ActiveModel {
            bna_id: ActiveValue::Set(bna_uuid),
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
            bna_id: ActiveValue::Set(bna_uuid.clone()),
            community_centers: ActiveValue::Set(scorecard.bna_recreation_community_centers),
            parks: ActiveValue::Set(scorecard.bna_recreation_parks),
            recreation_trails: ActiveValue::Set(scorecard.bna_recreation_trails),
            score: ActiveValue::Set(scorecard.bna_recreation),
        };
        bna_recreation.push(recreation_model);

        // Populate the opportunity model.
        let opportunity_model = opportunity::ActiveModel {
            bna_id: ActiveValue::Set(bna_uuid),
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
            bna_id: ActiveValue::Set(bna_uuid),
            low_stress_miles: ActiveValue::Set(scorecard.bna_total_low_stress_miles),
            high_stress_miles: ActiveValue::Set(scorecard.bna_total_high_stress_miles),
        };
        bna_infrastructure.push(infratructure_model);
    }

    // Insert the entries.
    City::insert_many(cities.into_values()).exec(&db).await?;
    Census::insert_many(census_populations).exec(&db).await?;
    SpeedLimit::insert_many(speed_limits).exec(&db).await?;
    Summary::insert_many(summaries).exec(&db).await?;
    People::insert_many(bna_people).exec(&db).await?;
    Retail::insert_many(bna_retail).exec(&db).await?;
    Transit::insert_many(bna_transit).exec(&db).await?;
    for chunk in bna_core_services.chunks(CHUNK_SIZE) {
        CoreServices::insert_many(chunk.to_vec()).exec(&db).await?;
    }
    dbg!(bna_core_services.len());
    for chunk in bna_recreation.chunks(CHUNK_SIZE) {
        Recreation::insert_many(chunk.to_vec()).exec(&db).await?;
    }
    for chunk in bna_opportunity.chunks(CHUNK_SIZE) {
        Opportunity::insert_many(chunk.to_vec()).exec(&db).await?;
    }
    for chunk in bna_infrastructure.chunks(CHUNK_SIZE) {
        Infrastructure::insert_many(chunk.to_vec())
            .exec(&db)
            .await?;
    }

    Ok(())
}
