use dotenv::dotenv;
use effortless::api::{entry_not_found, missing_parameter, parse_path_parameter};
use entity::{city, summary};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambdas::{build_paginated_response, database_connect, pagination_parameters};
use sea_orm::{prelude::Uuid, EntityTrait, PaginatorTrait};
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

    // Retrieve the ID of the entry to get if any.
    let id = match parse_path_parameter::<Uuid>(&event, "id") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    //
    match id {
        Some(id) => {
            let model = city::Entity::find_by_id(id)
                .find_also_related(summary::Entity)
                .paginate(&db, page_size)
                .fetch_page(page - 1)
                .await?;
            if model.is_empty() {
                return Ok(entry_not_found(&event).into());
            }
            let total_items = city::Entity::find().count(&db).await?;
            build_paginated_response(json!(model), total_items, page, page_size, &event)
        }
        None => Ok(missing_parameter(&event, "id").into()),
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
