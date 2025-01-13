use super::{
    adaptor::{get_price_fargate_adaptor_model_, get_prices_fargate_adaptor_model_},
    PriceParameters,
};
use crate::{
    core::resource::{
        price::schema::{FargatePrice, FargatePrices},
        schema::{ErrorResponses, PaginationParams},
    },
    PageFlow, Paginatron,
};
use crate::{Context, ExecutionError};
use axum::{
    extract::{Path, Query},
    Json,
};
use effortless::api::PaginationParameters;
use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "price";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_prices_fargate))
        .routes(routes!(get_price_fargate))
}

#[utoipa::path(
  get,
  path = "/prices/fargate",
  description = "Get all the AWS Fargate prices used to compute analysis costs.",
  tag = TAG,
  params(
    PaginationParams
  ),
  responses(
    (status = OK, description = "Fetches a collection of Fargate prices", body = FargatePrices),
  ))]
pub(crate) async fn get_prices_fargate(
    pagination: Option<Query<PaginationParameters>>,
    price_parameters: PriceParameters,
) -> Result<PageFlow<FargatePrices>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    let (total_items, models) = get_prices_fargate_adaptor_model_(
        price_parameters,
        pagination.page,
        pagination.page_size(),
    )
    .await?;
    let payload: FargatePrices = models.into();
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, pagination.page, pagination.page_size()),
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

// (status = BAD_REQUEST, description = "The request was formatted incorrectly or missing required parameters.", body = UserResponses::BadRequest),
// (status = NOT_FOUND, description = "The particular resource you are requesting was not found. This occurs, for example, if you request a resource by an id that does not exist.", body = APIErrorNotFound ),
