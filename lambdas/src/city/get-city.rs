use dotenv::dotenv;
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use once_cell::sync::OnceCell;
use sea_orm::{prelude::*, Database, DatabaseConnection, EntityTrait};
use serde_json::json;

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();
    let db = DATABASE_CONNECTION.get().unwrap();

    // Extract and convert the city parameter.
    let path_parameters = event.path_parameters();
    let city_id = match path_parameters.first("city_id") {
        Some(city_id) => Uuid::parse_str(city_id)?,
        None => return Err("No `city_id` parameter was provided".into()),
    };

    // Retrieve a specific city.
    Ok(json!(city::Entity::find_by_id(city_id).one(db).await?)
        .into_response()
        .await)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
