/// This example populates the database the database using the city ratings sample file.
use bnacore::{scorecard::scorecard23::ScoreCard23, versioning::Calver};
use color_eyre::{eyre::Report, Result};
use csv::Reader;
use dotenv::dotenv;
use entity::{
    census, city, core_services, features, infrastructure, opportunity, prelude::*, ranking,
    recreation, speed_limit, summary,
};
use sea_orm::{
    prelude::Uuid,
    ActiveValue::{self},
    Database, EntityTrait,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Populate entities.
    let mut cities: HashMap<Uuid, city::ActiveModel> = HashMap::new();
    let mut census_populations: Vec<census::ActiveModel> = Vec::new();
    let mut rankings: Vec<ranking::ActiveModel> = Vec::new();
    let mut speed_limits: Vec<speed_limit::ActiveModel> = Vec::new();
    let mut summaries: Vec<summary::ActiveModel> = Vec::new();
    let mut bna_features: Vec<features::ActiveModel> = Vec::new();
    let mut bna_core_services: Vec<core_services::ActiveModel> = Vec::new();
    let mut bna_recreation: Vec<recreation::ActiveModel> = Vec::new();
    let mut bna_opportunity: Vec<opportunity::ActiveModel> = Vec::new();
    let mut bna_infrastructure: Vec<infrastructure::ActiveModel> = Vec::new();
    let mut versions: HashMap<Uuid, Calver> = HashMap::new();

    // Load the CSV file.
    let mut csv_reader = Reader::from_path("examples/importer-sample.csv")?;
    for record in csv_reader.deserialize() {
        // Read the record.
        let scorecard: ScoreCard23 = record?;

        // Get the record's year.
        let year = scorecard.bna.year - 2000;
        let version = Calver::try_from_ubuntu(format!("{year}.1").as_str()).unwrap();

        // Get the City UUID.
        let city_uuid = Uuid::parse_str(&scorecard.city.bna_id).unwrap();

        // Look for a newer summary entry than this one.
        let mut has_newer = false;
        if versions.contains_key(&city_uuid) {
            let v = versions.get(&city_uuid).clone().unwrap();
            if version > *v {
                versions.insert(city_uuid.clone(), version.clone());
                has_newer = true;
            }
        } else {
            versions.insert(city_uuid.clone(), version.clone());
        }

        // Populate the city model.
        if !has_newer {
            let city_model = city::ActiveModel {
                city_id: ActiveValue::Set(city_uuid.clone()),
                country: ActiveValue::Set(scorecard.city.country.clone()),
                latitude: ActiveValue::Set(scorecard.city.census_latitude),
                longitude: ActiveValue::Set(scorecard.city.census_longitude),
                name: ActiveValue::Set(scorecard.city.city),
                region: ActiveValue::Set(scorecard.city.region),
                state: ActiveValue::Set(scorecard.city.state_full),
                state_abbrev: ActiveValue::Set(scorecard.city.state),
            };
            cities.insert(city_uuid.clone(), city_model);
        }

        // Populate the census population model.
        let census_model = census::ActiveModel {
            census_id: ActiveValue::NotSet,
            city_id: ActiveValue::Set(city_uuid.clone()),
            created_at: ActiveValue::NotSet,
            fips_code: ActiveValue::Set(scorecard.city.census_fips_code.to_string()),
            pop_size: match scorecard.bna.pop_size {
                bnacore::scorecard::scorecard23::Size::Small => ActiveValue::Set(0),
                bnacore::scorecard::scorecard23::Size::Medium => ActiveValue::Set(1),
                bnacore::scorecard::scorecard23::Size::Large => ActiveValue::Set(2),
            },
            population: ActiveValue::Set(scorecard.bna.census_population.try_into().unwrap()),
        };
        census_populations.push(census_model);

        // Populate the ranking model.
        let rankings_model = ranking::ActiveModel {
            ranking_id: ActiveValue::NotSet,
            city_id: ActiveValue::Set(city_uuid.clone()),
            country: ActiveValue::Set(scorecard.city.country.clone()),
            country_size: ActiveValue::Set(scorecard.bna.rank_country_size.try_into().unwrap()),
            created_at: ActiveValue::NotSet,
            global: ActiveValue::Set(scorecard.bna.rank.try_into().unwrap()),
            size: ActiveValue::Set(scorecard.bna.rank_size.try_into().unwrap()),
            state: ActiveValue::Set(scorecard.bna.rank_state.try_into().unwrap()),
        };
        rankings.push(rankings_model);

        // Populate the speed limit model.
        let speed_limit_model = speed_limit::ActiveModel {
            speed_limit_id: ActiveValue::NotSet,
            city_id: ActiveValue::Set(city_uuid.clone()),
            created_at: ActiveValue::NotSet,
            residential: ActiveValue::Set(
                scorecard.bna.residential_speed_limit.try_into().unwrap(),
            ),
        };
        speed_limits.push(speed_limit_model);

        // Populate the summary model.
        let bna_uuid = Uuid::parse_str(scorecard.bna.bna_uuid.as_str()).unwrap();
        let summary_model = summary::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            city_id: ActiveValue::Set(city_uuid.clone()),
            created_at: ActiveValue::NotSet,
            score: ActiveValue::Set(scorecard.bna.bna_rounded_score.try_into().unwrap()),
            version: ActiveValue::Set(version.to_ubuntu()),
        };
        summaries.push(summary_model);

        // Populate the features model.
        let feature_model = features::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            people: ActiveValue::Set(scorecard.bna.bna_people),
            retail: ActiveValue::Set(scorecard.bna.bna_retail),
            transit: ActiveValue::Set(scorecard.bna.bna_transit),
        };
        bna_features.push(feature_model);

        // Populate the Core Services model.
        let core_services_model = core_services::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            dentists: ActiveValue::Set(scorecard.bna.bna_core_services_dentists),
            doctors: ActiveValue::Set(scorecard.bna.bna_core_services_doctors),
            grocery: ActiveValue::Set(scorecard.bna.bna_core_services_grocery),
            hospitals: ActiveValue::Set(scorecard.bna.bna_core_services_hospitals),
            pharmacies: ActiveValue::Set(scorecard.bna.bna_core_services_pharmacies),
            score: ActiveValue::Set(scorecard.bna.bna_core_services),
            social_services: ActiveValue::Set(scorecard.bna.bna_core_services_social_services),
        };
        bna_core_services.push(core_services_model);

        // Populate the recreation model.
        let recreation_model = recreation::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            community_centers: ActiveValue::Set(scorecard.bna.bna_recreation_community_centers),
            parks: ActiveValue::Set(scorecard.bna.bna_recreation_parks),
            recreation_trails: ActiveValue::Set(scorecard.bna.bna_recreation_trails),
            score: ActiveValue::Set(scorecard.bna.bna_recreation),
        };
        bna_recreation.push(recreation_model);

        // Populate the opportunity model.
        let opportunity_model = opportunity::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            employment: ActiveValue::Set(scorecard.bna.bna_opportunity_employment),
            higher_education: ActiveValue::Set(scorecard.bna.bna_opportunity_higher_education),
            k12_education: ActiveValue::Set(scorecard.bna.bna_opportunity_k12_education),
            score: ActiveValue::Set(scorecard.bna.bna_opportunity),
            technical_vocational_college: ActiveValue::Set(
                scorecard.bna.bna_opportunity_technical_vocational_college,
            ),
        };
        bna_opportunity.push(opportunity_model);

        // Populate the infrastructure model.
        let infratructure_model = infrastructure::ActiveModel {
            bna_uuid: ActiveValue::Set(bna_uuid.clone()),
            low_stress_miles: ActiveValue::Set(scorecard.bna.bna_total_low_stress_miles),
            high_stress_miles: ActiveValue::Set(scorecard.bna.bna_total_high_stress_miles),
        };
        bna_infrastructure.push(infratructure_model);
    }

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    // Insert the entries.
    City::insert_many(cities.into_values()).exec(&db).await?;
    Census::insert_many(census_populations).exec(&db).await?;
    Ranking::insert_many(rankings).exec(&db).await?;
    SpeedLimit::insert_many(speed_limits).exec(&db).await?;
    Summary::insert_many(summaries).exec(&db).await?;
    Features::insert_many(bna_features).exec(&db).await?;
    CoreServices::insert_many(bna_core_services)
        .exec(&db)
        .await?;
    Recreation::insert_many(bna_recreation).exec(&db).await?;
    Opportunity::insert_many(bna_opportunity).exec(&db).await?;
    Infrastructure::insert_many(bna_infrastructure)
        .exec(&db)
        .await?;

    Ok(())
}
