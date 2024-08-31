use dotenv::dotenv;
use effortless::api::entry_not_found;
use entity::{city, summary};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{database_connect, ratings::extract_path_parameters};
use sea_orm::EntityTrait;
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve the parameters.
    let params = match extract_path_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e.into()),
    };

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve a bna result and its related city.
    let model = summary::Entity::find_by_id(params.bna_id)
        .find_also_related(city::Entity)
        .one(&db)
        .await?;
    let res: Response<Body> = match model {
        Some(model) => json!(model).into_response().await,
        None => entry_not_found(&event).into(),
    };
    Ok(res)
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

    run(service_fn(function_handler)).await.map_err(|e| {
        info!("{e}");
        e
    })
}
