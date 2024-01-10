use dotenv::dotenv;
use entity::{prelude::*, sea_orm_active_enums::ApprovalStatus};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use lambdas::{
    api_database_connect, build_paginated_response, get_apigw_request_id, pagination_parameters,
    APIError, APIErrors,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
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

    // Retrieve the API Gateway request ID if any.
    let apigw_request_id = get_apigw_request_id(&event);

    // Filter the query if needed.
    let mut conditions = Condition::all();
    let query_strings = event.query_string_parameters();

    // Retrieve the status parameter if available.
    let query_param_key = "status";
    let status_str = query_strings.first(query_param_key);
    if let Some(status) = status_str {
        match serde_plain::from_str::<ApprovalStatus>(status) {
            Ok(status) => {
                conditions = conditions.add(entity::submission::Column::Status.eq(status))
            }
            Err(e) => {
                let api_error = APIError::with_parameter(
                    apigw_request_id,
                    query_param_key,
                    format!("{query_param_key} is not a valid status: {e}").as_str(),
                );
                return Ok(APIErrors::new(&[api_error]).into());
            }
        }
    }

    // Retrieve all submissions or a specific one.
    debug!("Processing the requests...");
    let param_key = "submission_id";
    match event.path_parameters().first(param_key) {
        Some(submission_id_str) => {
            let submission_id = submission_id_str.parse::<i32>();
            match submission_id {
                Ok(submission_id) => {
                    let model = Submission::find_by_id(submission_id)
                        .filter(conditions)
                        .one(&db)
                        .await?;
                    let res: Response<Body> = match model {
                        Some(model) => json!(model).into_response().await,
                        None => {
                            let api_error = APIError::no_content(
                                apigw_request_id,
                                event.uri().path().to_string().as_str(),
                                format!(
                                    "submission entry with the id {submission_id} was not found"
                                )
                                .as_str(),
                            );
                            APIErrors::new(&[api_error]).into()
                        }
                    };
                    Ok(res)
                }
                Err(e) => {
                    let api_error = APIError::with_parameter(
                        apigw_request_id,
                        param_key,
                        format!("{submission_id_str} is not a valid id: {e}").as_str(),
                    );
                    Ok(APIErrors::new(&[api_error]).into())
                }
            }
        }
        None => {
            let query = Submission::find()
                .filter(conditions.clone())
                .paginate(&db, page_size)
                .fetch_page(page - 1)
                .await;
            let res: Response<Body> = match query {
                Ok(models) => {
                    let total_items = Submission::find().filter(conditions).count(&db).await?;
                    build_paginated_response(json!(models), total_items, page, page_size, &event)?
                }
                Err(e) => {
                    let api_error = APIError::with_pointer(
                        apigw_request_id,
                        event.uri().path().to_string().as_str(),
                        e.to_string().as_str(),
                    );
                    APIErrors::new(&[api_error]).into()
                }
            };
            Ok(res)
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
