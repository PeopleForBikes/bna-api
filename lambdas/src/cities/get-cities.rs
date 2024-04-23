use dotenv::dotenv;
use effortless::api::{entry_not_found, parse_path_parameter};
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
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
    let region = match parse_path_parameter::<String>(&event, "region") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };
    let name = match parse_path_parameter::<String>(&event, "name") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    if country.is_some() && region.is_some() && name.is_some() {
        let select = city::Entity::find_by_id((country.unwrap(), region.unwrap(), name.unwrap()));
        let model = select.one(&db).await?;
        let res: Response<Body> = match model {
            Some(model) => json!(model).into_response().await,
            None => entry_not_found(&event).into(),
        };
        Ok(res)
    } else {
        let select = city::Entity::find();
        let body = select
            .clone()
            .paginate(&db, page_size)
            .fetch_page(page - 1)
            .await?;
        let total_items = select.count(&db).await?;
        build_paginated_response(json!(body), total_items, page, page_size, &event)
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
