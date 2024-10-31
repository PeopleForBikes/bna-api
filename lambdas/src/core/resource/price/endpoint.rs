use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use effortless::api::PaginationParameters;
use serde_json::Value;

use crate::{Context, ExecutionError};

use super::{
    adaptor::{get_price_fargate_adaptor, get_prices_fargate_adaptor},
    PriceParameters,
};

pub fn routes() -> Router {
    Router::new()
        .route("/prices/fargate", get(get_prices_fargate))
        .route("/prices/fargate/:price_id", get(get_price_fargate))
}

pub async fn get_prices_fargate(
    pagination: Option<Query<PaginationParameters>>,
    price_parameters: PriceParameters,
) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    get_prices_fargate_adaptor(price_parameters, pagination.page, pagination.page_size()).await
}

pub async fn get_price_fargate(
    Path(price_id): Path<i32>,
    ctx: Context,
) -> Result<Json<Value>, ExecutionError> {
    get_price_fargate_adaptor(price_id, ctx).await.map(Json)
}
