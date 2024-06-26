use dotenv::dotenv;
use effortless::api::parse_request_body;
use entity::{prelude::*, wrappers::brokenspoke_pipeline::BrokenspokePipelinePost};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{EntityTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract and serialize the data.
    let wrapper = match parse_request_body::<BrokenspokePipelinePost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Turn the model wrapper into an active model.
    let active_model = wrapper.into_active_model();

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // And insert a new entry.
    info!(
        "inserting Brokenspoke pipeline into database: {:?}",
        active_model
    );
    let res = BrokenspokePipeline::insert(active_model).exec(&db).await?;
    Ok(json!(res.last_insert_id).into_response().await)
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
