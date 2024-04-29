use dotenv::dotenv;
use effortless::api::{entry_not_found, missing_parameter, parse_path_parameter};
use entity::{city, summary};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambdas::{build_paginated_response, database_connect, pagination_parameters};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve pagination parameters if any.
    let (page_size, page) = match pagination_parameters(&event) {
        Ok((page_size, page)) => (page_size, page),
        Err(e) => return Ok(e),
    };

    let country = match parse_path_parameter::<String>(&event, "country") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };
    dbg!(&country);
    let region = match parse_path_parameter::<String>(&event, "region") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };
    let name = match parse_path_parameter::<String>(&event, "name") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    if country.is_some() && region.is_some() && name.is_some() {
        let select = city::Entity::find_by_id((country.unwrap(), region.unwrap(), name.unwrap()))
            .find_also_related(summary::Entity);
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
    } else {
        Ok(missing_parameter(&event, "country or region or name").into())
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
