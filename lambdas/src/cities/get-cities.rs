use dotenv::dotenv;
use effortless::api::{entry_not_found, parse_path_parameter};
use entity::city;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{build_paginated_response, database_connect, pagination_parameters};
use sea_orm::{prelude::Uuid, EntityTrait, PaginatorTrait};
use serde_json::json;

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

    // Retrieve all cities or a specific one.
    match id {
        Some(id) => {
            let model = city::Entity::find_by_id(id).one(&db).await?;
            let res: Response<Body> = match model {
                Some(model) => json!(model).into_response().await,
                None => entry_not_found(&event).into(),
            };
            Ok(res)
        }
        None => {
            let body = city::Entity::find()
                .paginate(&db, page_size)
                .fetch_page(page - 1)
                .await?;
            let total_items = city::Entity::find().count(&db).await?;
            build_paginated_response(json!(body), total_items, page, page_size, &event)
        }
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

    run(service_fn(function_handler)).await
}
