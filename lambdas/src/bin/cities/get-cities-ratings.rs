use dotenv::dotenv;
use effortless::{api::extract_pagination_parameters, error::APIErrors, fragment::BnaRequestExt};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    core::resource::cities::{
        adaptor::get_cities_ratings_adaptor, extract_path_parameters, CitiesPathParameters,
    },
    Context,
};
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract the path parameters.
    let params: CitiesPathParameters = match extract_path_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e.into()),
    };

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };
    let ctx = Context::new(
        event.apigw_request_id(),
        event
            .uri()
            .path_and_query()
            .expect("to have a path and optional query parameters")
            .to_string(),
    );

    match get_cities_ratings_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
        ctx,
    )
    .await
    {
        Ok(v) => Ok(v.payload().into_response().await),
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

#[cfg(test)]
mod tests {
    // use super::*;
    // use lambda_http::{http, RequestExt};
    // use std::collections::HashMap;

    // #[tokio::test]
    // async fn test_handler_opportunity() {
    //     let event = http::Request::builder()
    //         .header(http::header::CONTENT_TYPE, "application/json")
    //         .body(Body::Empty)
    //         .expect("failed to build request")
    //         .with_path_parameters(HashMap::from([(
    //             "country".to_string(),
    //             "United%20States".to_string(),
    //         )]))
    //         .with_query_string_parameters(HashMap::from([(
    //             "region".to_string(),
    //             "Texas".to_string(),
    //         )]))
    //         .with_query_string_parameters(HashMap::from([(
    //             "name".to_string(),
    //             "Austin".to_string(),
    //         )]))
    //         .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
    //             lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    //         ));
    //     let r = function_handler(event).await.unwrap();
    //     dbg!(r);
    // }
}
