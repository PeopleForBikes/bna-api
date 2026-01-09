use super::adaptor::{get_price_fargate_adaptor_model_, get_prices_fargate_adaptor};
use crate::{
    core::resource::{
        price::schema::{FargatePrice, FargatePrices},
        schema::{ErrorResponses, ListParameters},
    },
    PageFlow, Paginatron,
};
use crate::{Context, ExecutionError};
use axum::{
    extract::{Path, Query},
    Json,
};
use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "price";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_prices_fargate))
        .routes(routes!(get_price_fargate))
}

#[axum::debug_handler]
#[utoipa::path(
  get,
  path = "/prices/fargate",
  description = "Get all the AWS Fargate prices used to compute analysis costs.",
  tag = TAG,
  params(
    ListParameters
  ),
  responses(
    (status = OK, description = "Fetches a collection of Fargate prices", body = FargatePrices),
  ))]
pub(crate) async fn get_prices_fargate(
    Query(list): Query<ListParameters>,
) -> Result<PageFlow<FargatePrices>, ExecutionError> {
    let (total_items, models) = get_prices_fargate_adaptor(
        list.order_direction(),
        &list.sort_by(),
        list.latest(),
        list.page(),
        list.page_size(),
    )
    .await?;
    let payload: FargatePrices = models.into();
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, list.page(), list.page_size()),
        payload,
    ))
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
    ErrorResponses,
  ))]
pub(crate) async fn get_price_fargate(
    Path(price_id): Path<i32>,
    ctx: Context,
) -> Result<Json<FargatePrice>, ExecutionError> {
    get_price_fargate_adaptor_model_(price_id, ctx)
        .await
        .map(FargatePrice::from)
        .map(Json)
}
