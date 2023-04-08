use dotenv::dotenv;
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use sea_orm::{Database, EntityTrait};
use serde_json::json;
use std::env;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    // Retrieve cities.
    Ok(json!(city::Entity::find().all(&db).await?)
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
