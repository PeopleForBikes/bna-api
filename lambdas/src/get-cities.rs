use dotenv::dotenv;
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde_json::json;

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();
    let db = DATABASE_CONNECTION.get().unwrap();

    // Retrieve cities.
    Ok(json!(city::Entity::find().all(db).await?)
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
