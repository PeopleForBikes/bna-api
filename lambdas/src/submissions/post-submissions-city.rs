use dotenv::dotenv;
use entity::submission;
use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request, Response,
};
use lambdas::{database_connect, get_apigw_request_id, APIError, APIErrorSource, APIErrors};
use sea_orm::{ActiveValue, EntityTrait};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Extract and serialize the data.
    let apigw_request_id = get_apigw_request_id(&event);
    let body = event.body();
    let body_str = std::str::from_utf8(body).expect("invalid utf-8 sequence");
    let model = match serde_json::from_str::<submission::Model>(body_str) {
        Ok(model) => model,
        Err(e) => {
            let api_error = APIError::new(
                apigw_request_id,
                StatusCode::BAD_REQUEST,
                String::from("Invalid data"),
                format!("The following submission is invalid: {body_str}. {e}"),
                APIErrorSource::Pointer(event.uri().path().to_string()),
            );
            return Ok(APIErrors::new(&[api_error]).into());
        }
    };

    // Turn the model into an active model and unset the primary key.
    let mut active_submission: submission::ActiveModel = model.into();
    active_submission.id = ActiveValue::NotSet;
    dbg!(&active_submission);

    // Insert the submission into the database.
    let res = submission::Entity::insert(active_submission)
        .exec(&db)
        .await?;
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

    run(service_fn(function_handler)).await
}
