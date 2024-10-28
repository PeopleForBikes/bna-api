use dotenv::dotenv;
use effortless::{api::extract_pagination_parameters, error::APIErrors, fragment::BnaRequestExt};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    core::resource::cities::{
        adaptor::{get_cities_adaptor, get_city_adaptor},
        extract_path_parameters, CitiesPathParameters,
    },
    Context,
};
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve a single city.
    if event.has_path_parameters() {
        let params: CitiesPathParameters = match extract_path_parameters(&event) {
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
        info!("{:#?}", params);
        match get_city_adaptor(&params.country, &params.region, &params.name, ctx).await {
            Ok(v) => return Ok(v.into_response().await),
            Err(e) => return Ok(APIErrors::from(e).into()),
        }
    }

    // Extract pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Retrieve all cities.
    match get_cities_adaptor(pagination.page, pagination.page_size()).await {
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
    use super::*;
    use lambda_http::{http, RequestExt};
    use std::collections::HashMap;

    #[test]
    fn test_extract_path_parameters() {
        let country: String = String::from("United States");
        let region: String = String::from("Texas");
        let name: String = String::from("Austin");
        let event = http::Request::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::Empty)
            .expect("failed to build request")
            .with_path_parameters(HashMap::from([
                ("country".to_string(), country.clone()),
                ("region".to_string(), region.clone()),
                ("name".to_string(), name.clone()),
            ]))
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let r = extract_path_parameters(&event).unwrap();
        assert_eq!(r.country, country);
        assert_eq!(r.region, region);
        assert_eq!(r.name, name);
    }

    // #[tokio::test]
    // async fn test_handler() {
    //     let country: String = String::from("United States");
    //     let region: String = String::from("Texas");
    //     let name: String = String::from("Austin");
    //     let event = http::Request::builder()
    //         .header(http::header::CONTENT_TYPE, "application/json")
    //         .body(Body::Empty)
    //         .expect("failed to build request")
    //         .with_path_parameters(HashMap::from([
    //             ("country".to_string(), country.clone()),
    //             ("region".to_string(), region.clone()),
    //             ("name".to_string(), name.clone()),
    //         ]))
    //         .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
    //             lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    //         ));

    //     let res = function_handler(event).await.unwrap();
    //     dbg!(res);
    // }
}
