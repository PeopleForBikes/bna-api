use dotenv::dotenv;
use effortless::{
    api::{entry_not_found, extract_pagination_parameters, internal_error},
    fragment::BnaRequestExt,
};
use entity::prelude::*;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{api_database_connect, bnas::extract_path_parameters, build_paginated_response};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;
use tracing::{debug, info};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = match api_database_connect(&event).await {
        Ok(db) => db,
        Err(e) => return Ok(e),
    };
    if event.has_path_parameters() {
        let params = match extract_path_parameters(&event) {
            Ok(p) => p,
            Err(e) => return Ok(e.into()),
        };

        // Retrieve a specific entry.
        debug!("Processing the requests...");
        let model = BrokenspokePipeline::find_by_id(params.bna_id)
            .one(&db)
            .await?;
        let res: Response<Body> = match model {
            Some(model) => json!(model).into_response().await,
            None => entry_not_found(&event).into(),
        };
        return Ok(res);
    }

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    //
    let select = BrokenspokePipeline::find();
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
        Err(e) => internal_error(&event, e.to_string().as_str()).into(),
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
