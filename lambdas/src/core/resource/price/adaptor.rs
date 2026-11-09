use crate::{
    core::resource::{
        price::db::{fetch_fargate_price, fetch_fargate_prices, fetch_latest_fargate_price},
        schema::OrderDirection,
    },
    Context, ExecutionError,
};
use sea_orm::{DatabaseConnection, DbErr};

pub async fn get_price_fargate_adaptor(
    db: &DatabaseConnection,
    id: i32,
    ctx: Context,
) -> Result<entity::fargate_price::Model, ExecutionError> {
    // Fetch the city model.
    let model = fetch_fargate_price(db, id).await?;
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a fargate price with ID {id}"),
        )),
    }
}

/// Returns the latest Fargate price.
///
/// The migrations insert the first price and no operation deletes one, so an
/// empty table is a broken database, answered as an internal error.
pub async fn get_price_fargate_latest_adaptor(
    db: &DatabaseConnection,
) -> Result<entity::fargate_price::Model, ExecutionError> {
    fetch_latest_fargate_price(db).await?.ok_or_else(|| {
        ExecutionError::DatabaseError(DbErr::RecordNotFound(
            "the fargate_price table is empty, although the migrations insert the first price"
                .to_string(),
        ))
    })
}

pub async fn get_prices_fargate_adaptor(
    db: &DatabaseConnection,
    sort_direction: OrderDirection,
    sort_by: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::fargate_price::Model>), ExecutionError> {
    // Fetch a page of cities.
    let (total_items, models) =
        fetch_fargate_prices(db, sort_direction, sort_by, page, page_size).await?;

    Ok((total_items, models))
}
