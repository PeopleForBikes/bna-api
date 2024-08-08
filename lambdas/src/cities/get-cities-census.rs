use dotenv::dotenv;
use effortless::api::entry_not_found;
use entity::{census, city};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambdas::{
    build_paginated_response,
    cities::{extract_path_parameters, CitiesPathParameters},
    database_connect, pagination_parameters,
};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;
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
    let (page_size, page) = match pagination_parameters(&event) {
        Ok((page_size, page)) => (page_size, page),
        Err(e) => return Ok(e),
    };

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve the city and associated census(es).
    let select = city::Entity::find_by_id((params.country, params.region, params.name))
        .find_also_related(census::Entity);
    let model = select
        .clone()
        .paginate(&db, page_size)
        .fetch_page(page - 1)
        .await?;
    if model.is_empty() {
        return Ok(entry_not_found(&event).into());
    }
    let total_items = select.count(&db).await?;
    build_paginated_response(json!(model), total_items, page, page_size, &event)
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
