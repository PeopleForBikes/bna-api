use aws_config::BehaviorVersion;
use bnacore::aws::get_aws_parameter;
use dotenv::dotenv;
use effortless::api::{internal_error, invalid_body, parse_request_body};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

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
    let enqueued_cities = match parse_request_body::<Vec<EnqueueCity>>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Prepare the AWS client.
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let bna_sqs_queue = match get_aws_parameter("BNA_SQS_QUEUE_URL").await {
        Ok(param) => param,
        Err(e) => {
            let message = format!("cannot retrieve the SQS Queue URL: {e}");
            info!(message);
            return Ok(internal_error(&event, &message).into());
        }
    };

    // Enqueue the messages.
    let mut count = 0;
    for enqueued_city in enqueued_cities {
        let enqueued_city_string = match serde_json::to_string(&enqueued_city) {
            Ok(s) => s,
            Err(e) => return Ok(invalid_body(&event, e.to_string().as_str()).into()),
        };
        let send_message = sqs_client
            .send_message()
            .queue_url(&bna_sqs_queue.parameter.value)
            .message_body(enqueued_city_string)
            .send()
            .await;
        if send_message.is_err() {
            let e = send_message.err().unwrap();
            let message = format!("cannot send the message to the SQS queue: {e}");
            info!(message);
            return Ok(internal_error(&event, &message).into());
        }
        count += 1;
    }

    // Return the number of messages successfully processed..
    Ok(json!({"messages": count}).into_response().await)
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
