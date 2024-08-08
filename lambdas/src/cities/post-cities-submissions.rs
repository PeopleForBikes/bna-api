use dotenv::dotenv;
use effortless::api::parse_request_body;
use entity::wrappers::submission::SubmissionPost;
use lambda_http::{
    http::{header, StatusCode},
    run, service_fn, Body, Error, Request, Response,
};
use lambdas::database_connect;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract and serialize the data.
    let wrapper = match parse_request_body::<SubmissionPost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Turn the model wrapper into an active model.
    let active_submission = wrapper.into_active_model();

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // And insert a new entry.
    info!(
        "inserting Submission into database: {:?}",
        active_submission
    );
    let submission = active_submission.insert(&db).await?;
    let response = Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::Text(json!(submission).to_string()))
        .expect("unable to build http::Response");
    Ok(response)
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
