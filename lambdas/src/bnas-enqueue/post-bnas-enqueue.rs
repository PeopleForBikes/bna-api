use aws_config::BehaviorVersion;
use aws_sdk_sqs::{self};
use bnacore::aws::get_aws_parameter;
use dotenv::dotenv;
use effortless::{api::parse_request_body, error::APIError, fragment::get_apigw_request_id};
use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct EnqueueCity {
    pub country: String,
    pub city: String,
    pub region: String,
    pub fips_code: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract and deserialize the data.
    let enqueued_city = match parse_request_body::<EnqueueCity>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Prepare the AWS client.
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let bna_sqs_queue = get_aws_parameter("BNA_SQS_QUEUE_URL").await?;

    // Enqueue the message.
    let _send_message = match sqs_client
        .send_message()
        .queue_url(bna_sqs_queue)
        .message_body(serde_json::to_string(&enqueued_city)?)
        .send()
        .await
    {
        Ok(message) => message,
        Err(e) => {
            let api_error = APIError::new(
                get_apigw_request_id(&event),
                StatusCode::BAD_REQUEST,
                "SQS Client Error",
                format!("cannot enqueue the message: {e}").as_str(),
                None,
            );
            return Ok(api_error.into());
        }
    };

    Ok(json!(enqueued_city).into_response().await)
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
