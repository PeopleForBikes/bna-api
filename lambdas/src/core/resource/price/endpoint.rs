use super::adaptor::{
    get_price_fargate_adaptor, get_price_fargate_latest_adaptor, get_prices_fargate_adaptor,
};
use crate::core::extract::{ApiPath, ApiQuery, PageUrl};
use crate::{
    core::resource::{
        price::schema::{FargatePrice, FargatePrices, PriceListParameters},
        schema::{BadRequestResponse, NotFoundResponse},
    },
    database_connect_or_init, PageFlow, Paginatron,
};
use crate::{Context, ExecutionError};
use axum::Json;
use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "price";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_prices_fargate))
        .routes(routes!(get_price_fargate_latest))
        .routes(routes!(get_price_fargate))
}

#[axum::debug_handler]
#[utoipa::path(
  get,
  path = "/prices/fargate",
  description = "Get all the AWS Fargate prices used to compute analysis costs.",
  tag = TAG,
  params(
    PriceListParameters
  ),
  responses(
    (status = OK, description = "Fetches a collection of Fargate prices", body = FargatePrices),
    BadRequestResponse,
  ))]
pub(crate) async fn get_prices_fargate(
    ApiQuery(list): ApiQuery<PriceListParameters>,
    PageUrl(url): PageUrl,
) -> Result<PageFlow<FargatePrices>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let (total_items, models) = get_prices_fargate_adaptor(
        db,
        list.order_direction(),
        &list.sort_by(),
        list.page(),
        list.page_size(),
    )
    .await?;
    let payload: FargatePrices = models.into();
    Ok(PageFlow::new(
        Paginatron::new(url, total_items, list.page(), list.page_size()),
        payload,
    ))
}

#[utoipa::path(
  get,
  path = "/prices/fargate/latest",
  description = "Get the latest AWS Fargate price, the one used to estimate the cost of a new analysis.",
  tag = TAG,
  responses(
    (status = OK, description = "Fetches the most recent Fargate price", body = FargatePrice),
  ))]
pub(crate) async fn get_price_fargate_latest() -> Result<Json<FargatePrice>, ExecutionError> {
    let db = database_connect_or_init().await?;
    get_price_fargate_latest_adaptor(db)
        .await
        .map(FargatePrice::from)
        .map(Json)
}

#[utoipa::path(
  get,
  path = "/prices/fargate/{price_id}",
  description = "Get a specific AWS Fargate price used to compute the cost of analysis cost.",
  tag = TAG,
  params(
    ("price_id" = i32, Path, description = "Identifier of a Fargate price")
  ),
  responses(
    (status = OK, description = "Fetches a Fargate price used to estimate the cost of an analysis", body = FargatePrice),
    BadRequestResponse,
    NotFoundResponse,
  ))]
pub(crate) async fn get_price_fargate(
    ApiPath(price_id): ApiPath<i32>,
    ctx: Context,
) -> Result<Json<FargatePrice>, ExecutionError> {
    let db = database_connect_or_init().await?;
    get_price_fargate_adaptor(db, price_id, ctx)
        .await
        .map(FargatePrice::from)
        .map(Json)
}
