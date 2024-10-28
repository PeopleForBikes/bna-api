use dotenv::dotenv;
use effortless::{api::extract_pagination_parameters, error::APIErrors};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::core::resource::cities::{
    adaptor::get_cities_censuses_adaptor, extract_path_parameters, CitiesPathParameters,
};
use tracing::info;

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

    match get_cities_censuses_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
    )
    .await
    {
        Ok(v) => Ok(v.payload().into_response().await),
        Err(e) => Ok(APIErrors::from(e).into()),
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use aws_lambda_events::http;
//     use lambda_http::RequestExt;
//     use std::collections::HashMap;

//     #[tokio::test]
//     async fn test_handler() {
//         let event = http::Request::builder()
//             .header(http::header::CONTENT_TYPE, "application/json")
//             .body(Body::Empty)
//             .expect("failed to build request")
//             .with_path_parameters(HashMap::from([(
//                 "id".to_string(),
//                 "c49fa94e-542d-421f-9826-e233538be929".to_string(), // Santa Monica, CA
//             )]))
//             .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
//                 lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
//             ));
//         let r = function_handler(event).await.unwrap();
//         dbg!(r);
//     }
// }
