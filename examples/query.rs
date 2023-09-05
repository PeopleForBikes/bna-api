/// This example queries all the cities in the database and displays some
/// information about each of them.
use color_eyre::{eyre::Report, Result};
use dotenv::dotenv;
use entity::city;
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();
    let db = DATABASE_CONNECTION.get().unwrap();

    // Retrieve cities and display some info.
    match city::Entity::find().all(db).await {
        Ok(city_models) => {
            for city_model in city_models {
                println!(
                    "The city of {} with the id {} is localted in {}.",
                    city_model.name, city_model.city_id, city_model.state
                )
            }
        }
        Err(error) => panic!("Cannot connect to the database: {}.", error.to_string()),
    }

    Ok(())
}
