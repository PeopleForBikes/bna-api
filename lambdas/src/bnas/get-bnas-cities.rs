use dotenv::dotenv;
use entity::{bna, city};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use lambdas::database_connect;
use sea_orm::{prelude::Uuid, EntityTrait};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve a bna result and its related city.
    match event.path_parameters().first("bna_id") {
        Some(bna_id) => Ok(json!(
            bna::Entity::find_by_id(bna_id.parse::<Uuid>()?)
                .find_also_related(city::Entity)
                .one(&db)
                .await?
        )
        .into_response()
        .await),
        None => Err("The `bna_id` parameter is missing.".into()),
    }
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
