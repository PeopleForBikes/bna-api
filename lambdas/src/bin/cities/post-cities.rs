use dotenv::dotenv;
use effortless::{api::parse_request_body, error::APIErrors};
use entity::wrappers::city::CityPost;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::core::resource::cities::adaptor::post_cities_adaptor;
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

    // Extract and serialize the data.
    let wrapper = match parse_request_body::<CityPost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    match post_cities_adaptor(wrapper).await {
        Ok(v) => Ok(v.into_response().await),
        Err(e) => Ok(APIErrors::from(e).into()),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use lambda_http::{http, RequestExt};

    // #[tokio::test]
    // async fn test_handler() {
    //     let event = http::Request::builder()
    //   .header(http::header::CONTENT_TYPE, "application/json")
    //   .body(Body::from(r#"{"name": "santa rosa 000","country": "united states","fips_code": "3570670","state": "new mexico", "state_abbrev": "NM"}"#))
    //   .expect("failed to build request")
    //   .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default()));

    //     let r = function_handler(event).await.unwrap();
    //     dbg!(r);
    // }
}
