use dotenv::dotenv;
use entity::{bna, city};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use lambdas::{database_connect, pagination_parameters};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve pagination parameters if any.
    let (page_size, page) = pagination_parameters(&event)?;

    // Retrieve a city and all its BNAs.
    match event.path_parameters().first("city_id") {
        Some(city_id) => Ok(json!(
            city::Entity::find_by_id(city_id.parse::<i32>()?)
                .find_also_related(bna::Entity)
                .paginate(&db, page_size)
                .fetch_page(page)
                .await?
        )
        .into_response()
        .await),
        None => Err("The `city_id` parameter is missing.".into()),
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
