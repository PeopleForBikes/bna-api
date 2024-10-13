use dotenv::dotenv;
use effortless::{
    api::{extract_pagination_parameters, parse_path_parameter, parse_query_string_parameter},
    error::APIErrors,
};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::cities::adaptor::{get_cities_submission_adaptor, get_cities_submissions_adaptor};
use tracing::{debug, info};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Retrieve the status parameter if available.
    let query_param_key = "status";
    let status = match parse_query_string_parameter::<String>(&event, query_param_key) {
        Ok(status) => status,
        Err(e) => return Ok(e.into()),
    };

    // Retrieve the ID of the Submission to get if any.
    let submission_id = match parse_path_parameter::<i32>(&event, "id") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Retrieve all submissions or a specific one.
    debug!("Processing the requests...");
    match submission_id {
        Some(id) => match get_cities_submission_adaptor(id, status).await {
            Ok(v) => return Ok(v.into_response().await),
            Err(e) => return Ok(APIErrors::from(e).into()),
        },
        None => {
            match get_cities_submissions_adaptor(status, pagination.page, pagination.page_size())
                .await
            {
                Ok(v) => return Ok(v.payload().into_response().await),
                Err(e) => return Ok(APIErrors::from(e).into()),
            }
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
