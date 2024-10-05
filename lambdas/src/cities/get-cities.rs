use dotenv::dotenv;
use effortless::{
    api::{entry_not_found, extract_pagination_parameters},
    fragment::BnaRequestExt,
};
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    build_paginated_response,
    cities::{extract_path_parameters, CitiesPathParameters},
    database_connect,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve a single city.
    if event.has_path_parameters() {
        let params: CitiesPathParameters = match extract_path_parameters(&event) {
            Ok(p) => p,
            Err(e) => return Ok(e.into()),
        };
        info!("{:#?}", params);
        return get_city_handler(&params.country, &params.region, &params.name, event).await;
    }

    // Retrieve all cities.
    get_cities_handler(event).await
}

async fn get_city_handler(
    country: &str,
    region: &str,
    name: &str,
    event: Request,
) -> Result<Response<Body>, Error> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the city model.
    let model = fetch_city(&db, country, region, name).await?;
    let res: Response<Body> = match model {
        Some(model) => json!(model).into_response().await,
        None => entry_not_found(&event).into(),
    };
    return Ok(res);
}

async fn get_cities_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of cities.
    let (total_items, body) = fetch_cities(&db, pagination.page, pagination.page_size).await?;

    // Return the paginated response.
    build_paginated_response(
        json!(body),
        total_items,
        pagination.page,
        pagination.page_size,
        &event,
    )
}

async fn fetch_city(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
) -> Result<Option<city::Model>, sea_orm::DbErr> {
    city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
        .one(db)
        .await
}

async fn fetch_cities(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<city::Model>), sea_orm::DbErr> {
    let select = city::Entity::find();
    let count = select
        .clone()
        .select_only()
        .column_as(city::Column::Id.count(), "count")
        .count(db)
        .await?;
    let models = select.paginate(db, page_size).fetch_page(page).await?;
    Ok((count, models))
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
