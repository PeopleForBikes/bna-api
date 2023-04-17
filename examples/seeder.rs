/// This example populates the database the database using the city ratings sample file.
use color_eyre::{eyre::Report, Result};
use csv::Reader;
use dotenv::dotenv;
use entity::{bna, city, prelude::*};
use itertools::Itertools;
use sea_orm::{prelude::Uuid, ActiveValue::Set, Database, EntityTrait, IntoActiveModel};
use serde::Deserialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Populate entities.
    let mut bnas: HashMap<Uuid, bna::ActiveModel> = HashMap::new();
    let mut cities: HashMap<i32, city::ActiveModel> = HashMap::new();

    // Load the CSV file.
    let mut csv_reader = Reader::from_path("examples/sample.csv")?;
    for record in csv_reader.deserialize() {
        // Read the record.
        let scorecard: ScoreCard = record?;
        let city_id = scorecard.city.census_fips_code;
        let bna_id = scorecard.bna.bna_uuid;

        // Extract the city.
        let active_city = scorecard.city.into_active_model();
        cities.insert(city_id, active_city);

        // Extract the BNA.
        let mut active_bna = scorecard.bna.into_active_model();
        active_bna.city_id = Set(Some(city_id));
        bnas.insert(bna_id, active_bna);
    }

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    // Insert the entries.
    City::insert_many(cities.into_values()).exec(&db).await?;
    Bna::insert_many(bnas.into_values().collect_vec())
        .exec(&db)
        .await?;

    Ok(())
}

/// Represent a city scorecard.
#[derive(Debug, Deserialize, Clone)]
pub struct ScoreCard {
    /// City details.
    #[serde(flatten)]
    pub city: city::Model,
    /// BNA results.
    #[serde(flatten)]
    pub bna: bna::Model,
}
