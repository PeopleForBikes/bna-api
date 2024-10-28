use dotenv::dotenv;
use effortless::{error::APIErrors, fragment::BnaRequestExt};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    core::resource::ratings::{adaptor::get_ratings_city_adaptor, extract_path_parameters},
    Context,
};
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve the parameters.
    let params = match extract_path_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e.into()),
    };
    let ctx = Context::new(
        event.apigw_request_id(),
        event
            .uri()
            .path_and_query()
            .expect("to have a path and optional query parameters")
            .to_string(),
    );

    match get_ratings_city_adaptor(params.bna_id, ctx).await {
        Ok(v) => Ok(v.into_response().await),
        Err(e) => Ok(APIErrors::from(e).into()),
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
