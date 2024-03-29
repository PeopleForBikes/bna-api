use aws_config::BehaviorVersion;
use dotenv::dotenv;
use effortless::{error::APIError, fragment::get_apigw_request_id};
use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

const BUCKET_NAME: &str = "brokenspoke-analyzer";

#[derive(Deserialize, Serialize, Debug)]
pub struct BNAResults {
    results: Vec<String>,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Configure the AWS S3 client.
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    // Collect the folders in the bucket.
    let objects = s3_client.list_objects_v2().bucket(BUCKET_NAME).send().await;
    match objects {
        Ok(o) => {
            let mut bna_results = o
                .contents()
                .iter()
                .filter(|o| o.key.clone().unwrap().ends_with('/'))
                .map(|o| o.key.clone().unwrap())
                .collect::<Vec<String>>();
            bna_results.sort();
            Ok(json!(bna_results).into_response().await)
        }
        Err(e) => {
            let api_error = APIError::new(
                get_apigw_request_id(&event),
                StatusCode::BAD_REQUEST,
                "S3 Client Error",
                format!("Cannot retrieve the content of the {BUCKET_NAME} bucket: {e}").as_str(),
                None,
            );
            Ok(api_error.into())
        }
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

    run(service_fn(function_handler)).await.map_err(|e| {
        info!("{e}");
        e
    })
}
