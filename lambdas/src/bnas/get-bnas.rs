use dotenv::dotenv;
use entity::bna;
use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response,
};
use lambdas::{
    build_paginated_response, database_connect, pagination_parameters, APIError, APIErrorSource,
    APIErrors,
};
use sea_orm::{prelude::Uuid, EntityTrait, PaginatorTrait};
use serde_json::json;
use tracing::{debug, error};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    debug!("Connecting to the database...");
    let db = match database_connect(Some("DATABASE_URL_SECRET_ID")).await {
        Ok(db) => db,
        Err(e) => {
            let error_msg = "cannot connect to the database".to_string();
            error!("{error_msg}: {e}");
            let api_error = APIError::db_error(event.uri().path(), &error_msg);
            return Ok(APIErrors::new(&[api_error]).into());
        }
    };

    // Retrieve pagination parameters if any.
    debug!("Retrieving pagination...");
    let (page_size, page) = match pagination_parameters(&event) {
        Ok((page_size, page)) => (page_size, page),
        Err(e) => return Ok(e),
    };

    // Retrieve all bnas or a specific one.
    debug!("Processing the requests...");
    match event.path_parameters().first("bna_id") {
        Some(bna_id_str) => {
            let bna_id = bna_id_str.parse::<Uuid>();
            match bna_id {
                Ok(bna_id) => {
                    let model = bna::Entity::find_by_id(bna_id).one(&db).await?;
                    let res: Response<Body> = match model {
                        None => {
                            let api_error = APIError::new(
                                StatusCode::NOT_FOUND,
                                String::from("Content Not Found"),
                                format!("BNA entry with the id {bna_id} was not found."),
                                APIErrorSource::Pointer(event.uri().path().to_string()),
                            );
                            APIErrors::new(&[api_error]).into()
                        }
                        Some(model) => json!(model).into_response().await,
                    };
                    Ok(res)
                }
                Err(e) => {
                    let api_error = APIError::with_parameter(
                        "bna_id",
                        format!("{bna_id_str} is not a valid UUID: {e}").as_str(),
                    );
                    Ok(APIErrors::new(&[api_error]).into())
                }
            }
        }
        None => {
            let body = bna::Entity::find()
                .paginate(&db, page_size)
                .fetch_page(page - 1)
                .await?;
            let total_items = bna::Entity::find().count(&db).await?;
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
