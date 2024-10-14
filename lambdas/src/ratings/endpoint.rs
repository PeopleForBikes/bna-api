use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use effortless::api::PaginationParameters;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::cities::ExecutionError;

use super::adaptor::{
    get_ratings_analyses_adaptor, get_ratings_analysis_adaptor, get_ratings_city_adaptor,
    get_ratings_summaries_adaptor, get_ratings_summary_adaptor,
};

pub fn routes() -> Router {
    Router::new()
        .route("/ratings/analyses", get(get_ratings_analyses))
        .route("/ratings/analyses/:analysis_id", get(get_ratings_analysis))
        .route("/ratings", get(get_ratings))
        .route("/ratings/:bna_id", get(get_rating))
        .route("/ratings/:bna_id/city", get(get_ratings_city))
}

async fn get_ratings_analyses(
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_ratings_analyses_adaptor(pagination.page, pagination.page_size())
        .await
        .map(|v| Json(json!(v.payload())))
}

async fn get_ratings_analysis(
    Path(analysis_id): Path<Uuid>,
) -> Result<Json<Value>, ExecutionError> {
    get_ratings_analysis_adaptor(analysis_id)
        .await
        .map(|v| Json(v))
}

async fn get_ratings(
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_ratings_summaries_adaptor(pagination.page, pagination.page_size())
        .await
        .map(|v| Json(json!(v.payload())))
}

async fn get_rating(Path(rating_id): Path<Uuid>) -> Result<Json<Value>, ExecutionError> {
    get_ratings_summary_adaptor(rating_id)
        .await
        .map(|v| Json(json!(v)))
}

async fn get_ratings_city(Path(bna_id): Path<Uuid>) -> Result<Json<Value>, ExecutionError> {
    get_ratings_city_adaptor(bna_id).await.map(|v| Json(v))
}
