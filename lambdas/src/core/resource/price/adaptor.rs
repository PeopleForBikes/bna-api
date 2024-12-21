use super::PriceParameters;
use crate::{
    core::resource::price::db::{fetch_fargate_price, fetch_fargate_prices},
    database_connect, Context, ExecutionError, PageFlow, Paginatron,
};
use entity::fargate_price;
use serde_json::{json, Value};

pub async fn get_price_fargate_adaptor(id: i32, ctx: Context) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the city model.
    let model = fetch_fargate_price(&db, id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a fargate price with ID {id}"),
        )),
    }
}

pub async fn get_price_fargate_adaptor_model_(
    id: i32,
    ctx: Context,
) -> Result<entity::fargate_price::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the city model.
    let model = fetch_fargate_price(&db, id).await?;
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a fargate price with ID {id}"),
        )),
    }
}

pub async fn get_prices_fargate_adaptor(
    params: PriceParameters,
    page: u64,
    page_size: u64,
) -> Result<PageFlow<Vec<fargate_price::Model>>, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of cities.
    let (total_items, models) = fetch_fargate_prices(
        &db,
        params.sort.unwrap_or_default(),
        params.latest,
        page,
        page_size,
    )
    .await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        models,
    ))
}

pub async fn get_prices_fargate_adaptor_model_(
    params: PriceParameters,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::fargate_price::Model>), ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of cities.
    let (total_items, models) = fetch_fargate_prices(
        &db,
        params.sort.unwrap_or_default(),
        params.latest,
        page,
        page_size,
    )
    .await?;

    Ok((total_items, models))
}
