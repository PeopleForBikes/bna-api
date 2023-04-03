/// This example populates the database the database using the city ratings sample file.
use color_eyre::{eyre::Report, Result};
use csv::Reader;
use dotenv::dotenv;
use entity::{bna, city, infrastructure, prelude::*};
use once_cell::sync::OnceCell;
use sea_orm::{
    prelude::*, ActiveValue::Set, Database, DatabaseConnection, EntityTrait, IntoActiveModel,
};
use serde::Deserialize;

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Populate entities.
    let mut bnas: Vec<bna::ActiveModel> = Vec::new();
    let mut cities: Vec<city::ActiveModel> = Vec::new();
    let mut infrastructures: Vec<infrastructure::ActiveModel> = Vec::new();

    // Load the CSV file.
    let mut csv_reader = Reader::from_path("examples/sample.csv")?;
    for record in csv_reader.deserialize() {
        // Read the record.
        let scorecard: ScoreCard = record?;
        let city_id = scorecard.bna_uuid;

        // Extract the city.
        let mut active_city = scorecard.city.into_active_model();
        active_city.id = Set(city_id);
        cities.push(active_city);

        // Extract the BNA.
        let mut active_bna = scorecard.bna.into_active_model();
        active_bna.city_id = Set(Some(city_id));
        bnas.push(active_bna);

        // Extract the Infrastructure.
        let mut active_infrastructure = scorecard.infrastructure.into_active_model();
        active_infrastructure.city_id = Set(Some(city_id));
        infrastructures.push(active_infrastructure);
    }

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();
    let db = DATABASE_CONNECTION.get().unwrap();

    // Insert the entries.
    City::insert_many(cities).exec(db).await?;
    Bna::insert_many(bnas).exec(db).await?;
    Infrastructure::insert_many(infrastructures)
        .exec(db)
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
    /// Infrastructure details.
    #[serde(flatten)]
    pub infrastructure: infrastructure::Model,
    /// BNA run unique identifier.
    pub bna_uuid: Uuid,
}
