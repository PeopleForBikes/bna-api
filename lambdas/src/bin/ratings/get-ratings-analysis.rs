use dotenv::dotenv;
use effortless::{api::extract_pagination_parameters, error::APIErrors, fragment::BnaRequestExt};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    core::resource::ratings::{
        adaptor::{get_ratings_analyses_adaptor, get_ratings_analysis_adaptor},
        extract_path_parameters,
    },
    Context,
};
use tracing::{debug, info};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    if event.has_path_parameters() {
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

        // Retrieve a specific entry.
        debug!("Processing the requests...");
        match get_ratings_analysis_adaptor(params.rating_id, ctx).await {
            Ok(v) => return Ok(v.into_response().await),
            Err(e) => return Ok(APIErrors::from(e).into()),
        }
    }

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    match get_ratings_analyses_adaptor(pagination.page, pagination.page_size()).await {
        Ok(v) => Ok(v.into()),
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
