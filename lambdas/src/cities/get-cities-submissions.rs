use dotenv::dotenv;
use effortless::{
    api::{
        entry_not_found, extract_pagination_parameters, parse_path_parameter,
        parse_query_string_parameter,
    },
    error::{APIError, APIErrors},
    fragment::get_apigw_request_id,
};
use entity::prelude::*;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{api_database_connect, build_paginated_response};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use serde_json::json;
use tracing::{debug, info};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = match api_database_connect(&event).await {
        Ok(db) => db,
        Err(e) => return Ok(e),
    };

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Filter the query if needed.
    let mut conditions = Condition::all();

    // Retrieve the status parameter if available.
    let query_param_key = "status";
    match parse_query_string_parameter::<String>(&event, query_param_key) {
        Ok(status) => {
            if let Some(status) = status {
                conditions = conditions.add(entity::submission::Column::Status.eq(status))
            }
        }
        Err(e) => return Ok(e.into()),
    }

    // Retrieve the ID of the Submission to get if any.
    let submission_id = match parse_path_parameter::<i32>(&event, "id") {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Retrieve all submissions or a specific one.
    debug!("Processing the requests...");
    let res = match submission_id {
        Some(id) => {
            let model = Submission::find_by_id(id)
                .filter(conditions)
                .one(&db)
                .await?;
            let res: Response<Body> = match model {
                Some(model) => json!(model).into_response().await,
                None => entry_not_found(&event).into(),
            };
            res
        }
        None => {
            let select = Submission::find().filter(conditions);
            let query = select
                .clone()
                .paginate(&db, pagination.page_size)
                .fetch_page(pagination.page)
                .await;
            let res: Response<Body> = match query {
                Ok(models) => {
                    let total_items = select.count(&db).await?;
                    build_paginated_response(
                        json!(models),
                        total_items,
                        pagination.page,
                        pagination.page_size,
                        &event,
                    )?
                }
                Err(e) => {
                    let api_error = APIError::with_pointer(
                        get_apigw_request_id(&event),
                        event.uri().path().to_string().as_str(),
                        e.to_string().as_str(),
                    );
                    APIErrors::new(&[api_error]).into()
                }
            };
            res
        }
    };

    Ok(res)
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
