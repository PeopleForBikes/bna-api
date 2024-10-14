use dotenv::dotenv;
use effortless::error::APIErrors;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::ratings::{adaptor::get_ratings_city_adaptor, extract_path_parameters};
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve the parameters.
    let params = match extract_path_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e.into()),
    };

    match get_ratings_city_adaptor(params.bna_id).await {
        Ok(v) => return Ok(v.into_response().await),
        Err(e) => return Ok(APIErrors::from(e).into()),
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
