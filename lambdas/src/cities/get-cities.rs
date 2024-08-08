use dotenv::dotenv;
use effortless::{api::entry_not_found, fragment::BnaRequestExt};
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    build_paginated_response,
    cities::{extract_path_parameters, PathParameters},
    database_connect, pagination_parameters_2,
};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // With params.
    if event.has_path_parameters() {
        let params: PathParameters = match extract_path_parameters(&event) {
            Ok(p) => p,
            Err(e) => return Ok(e.into()),
        };

        let select = city::Entity::find_by_id((params.country, params.region, params.name));
        let model = select.one(&db).await?;
        let res: Response<Body> = match model {
            Some(model) => json!(model).into_response().await,
            None => entry_not_found(&event).into(),
        };
        return Ok(res);
    }

    // Retrieve pagination parameters if any.
    let pagination = match pagination_parameters_2(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Without params.
    let select = city::Entity::find();
    let body = select
        .clone()
        .paginate(&db, pagination.page_size)
        .fetch_page(pagination.page - 1)
        .await?;
    let total_items = select.count(&db).await?;
    build_paginated_response(
        json!(body),
        total_items,
        pagination.page,
        pagination.page_size,
        &event,
    )
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
}
