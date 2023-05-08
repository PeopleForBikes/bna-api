use dotenv::dotenv;
use entity::{bna, city};
use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, RequestExt, Response};
use lambdas::{
    build_paginated_response, database_connect, pagination_parameters, APIError, APIErrorSource,
    APIErrors,
};
use sea_orm::{EntityTrait, PaginatorTrait};
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

    // Retrieve a city and all its BNAs.
    match event.path_parameters().first("city_id") {
        Some(city_id_str) => {
            let city_id = city_id_str.parse::<i32>();
            match city_id {
                Ok(city_id) => {
                    let model = city::Entity::find_by_id(city_id)
                        .find_also_related(bna::Entity)
                        .paginate(&db, page_size)
                        .fetch_page(page - 1)
                        .await?;
                    if model.is_empty() {
                        let api_error = APIError::new(
                            StatusCode::NOT_FOUND,
                            String::from("Content Not Found"),
                            format!("City entry with the id {city_id} was not found."),
                            APIErrorSource::Pointer(event.uri().path().to_string()),
                        );
                        return Ok(APIErrors::new(&[api_error]).into());
                    }
                    let total_items = city::Entity::find().count(&db).await?;
                    build_paginated_response(json!(model), total_items, page, page_size, &event)
                }
                Err(e) => {
                    let api_error = APIError::with_parameter(
                        "city_id",
                        format!("{city_id_str} is not a valid city id: {e}").as_str(),
                    );
                    Ok(APIErrors::new(&[api_error]).into())
                }
            }
        }
        None => {
            let api_error = APIError::with_parameter("city_id", "Parameter is missing.");
            Ok(APIErrors::new(&[api_error]).into())
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
