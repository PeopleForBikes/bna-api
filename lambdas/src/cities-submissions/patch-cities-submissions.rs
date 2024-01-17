use dotenv::dotenv;
use effortless::api::{missing_parameter, parse_path_parameter, parse_request_body};
use entity::{prelude::*, wrappers};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{ActiveValue, EntityTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve the ID of the Submission to update.
    let parameter = "id";
    let submission_id = match parse_path_parameter::<i32>(&event, parameter) {
        Ok(value) => match value {
            Some(v) => v,
            None => {
                return Ok(missing_parameter(&event, parameter).into());
            }
        },
        Err(e) => return Ok(e.into()),
    };

    // Extract and deserialize the data.
    let wrapper = match parse_request_body::<wrappers::Submission>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Turn the wrapper into an active model.
    let mut active_submission = wrapper.into_active_model();
    active_submission.id = ActiveValue::Set(submission_id);
    info!(
        "updating Submission {submission_id} into database: {:?}",
        active_submission
    );

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Update the Submission entry.
    let res = Submission::update(active_submission).exec(&db).await?;
    Ok(json!(res.id).into_response().await)
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
