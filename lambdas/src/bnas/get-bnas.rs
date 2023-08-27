use dotenv::dotenv;
use entity::summary;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use lambdas::{
    api_database_connect, build_paginated_response, get_apigw_request_id, pagination_parameters,
    APIError, APIErrors,
};
use sea_orm::{prelude::Uuid, EntityTrait, PaginatorTrait};
use serde_json::json;
use tracing::debug;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = match api_database_connect(&event).await {
        Ok(db) => db,
        Err(e) => return Ok(e),
    };

    // Retrieve pagination parameters if any.
    let (page_size, page) = match pagination_parameters(&event) {
        Ok((page_size, page)) => (page_size, page),
        Err(e) => return Ok(e),
    };

    // Retrieve all bnas or a specific one.
    debug!("Processing the requests...");
    let apigw_request_id = get_apigw_request_id(&event);
    match event.path_parameters().first("bna_id") {
        Some(bna_id_str) => {
            let bna_id = bna_id_str.parse::<Uuid>();
            match bna_id {
                Ok(bna_id) => {
                    let model = summary::Entity::find_by_id(bna_id).one(&db).await?;
                    let res: Response<Body> = match model {
                        Some(model) => json!(model).into_response().await,
                        None => {
                            let api_error = APIError::no_content(
                                apigw_request_id,
                                event.uri().path().to_string().as_str(),
                                format!("BNA entry with the id {bna_id} was not found.").as_str(),
                            );
                            APIErrors::new(&[api_error]).into()
                        }
                    };
                    Ok(res)
                }
                Err(e) => {
                    let api_error = APIError::with_parameter(
                        apigw_request_id,
                        "bna_id",
                        format!("{bna_id_str} is not a valid UUID: {e}").as_str(),
                    );
                    Ok(APIErrors::new(&[api_error]).into())
                }
            }
        }
        None => {
            let body = summary::Entity::find()
                .paginate(&db, page_size)
                .fetch_page(page - 1)
                .await?;
            let total_items = summary::Entity::find().count(&db).await?;
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
