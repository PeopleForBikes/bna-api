use dotenv::dotenv;
use effortless::{
    api::{
        entry_not_found, extract_pagination_parameters, invalid_path_parameter,
        parse_query_string_parameter,
    },
    error::APIError,
    fragment::{get_apigw_request_id, BnaRequestExt},
};
use entity::prelude::*;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{api_database_connect, build_paginated_response};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};
use serde_json::json;
use tracing::{debug, info};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Retrieve the ID of the entry to update.
    let parameter = "id";
    let id = event.path_parameter::<i32>(parameter);

    // Set the database connection.
    let db = match api_database_connect(&event).await {
        Ok(db) => db,
        Err(e) => return Ok(e),
    };

    // Retrieve all entries or a specific one.
    debug!("Processing the requests...");
    let res = match id {
        Some(id) => match id {
            Ok(id) => {
                let model = FargatePrice::find_by_id(id).one(&db).await?;
                let res: Response<Body> = match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                };
                res
            }
            Err(e) => {
                return Ok(
                    invalid_path_parameter(&event, parameter, e.to_string().as_str()).into(),
                );
            }
        },
        None => {
            // Retrieve the filter if any.
            let latest: Option<bool> = match parse_query_string_parameter::<bool>(&event, "latest")
            {
                Ok(latest) => latest,
                Err(e) => return Ok(e.into()),
            };

            // Select the entity.
            let mut select = FargatePrice::find();

            // Select latest only.
            if latest.is_some() {
                select = select
                    .order_by_asc(entity::fargate_price::Column::CreatedAt)
                    .limit(1);
            }

            // Select the results.
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
                Err(e) => APIError::with_pointer(
                    get_apigw_request_id(&event),
                    event.uri().path().to_string().as_str(),
                    e.to_string().as_str(),
                )
                .into(),
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
