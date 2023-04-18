use dotenv::dotenv;
use entity::bna;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use lambdas::{database_connect, pagination_parameters};
use sea_orm::{prelude::Uuid, EntityTrait, PaginatorTrait};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve pagination parameters if any.
    let (page_size, page) = pagination_parameters(&event)?;

    // Retrieve city or cities.
    match event.path_parameters().first("bna_id") {
        Some(bna_id) => {
            info!("/bnas/{bna_id}");
            Ok(json!(
                bna::Entity::find_by_id(bna_id.parse::<Uuid>()?)
                    .one(&db)
                    .await?
            )
            .into_response()
            .await)
        }
        None => {
            info!("/bnas");
            Ok(json!(
                bna::Entity::find()
                    .paginate(&db, page_size)
                    .fetch_page(page)
                    .await?
            )
            .into_response()
            .await)
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
