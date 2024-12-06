use super::{
    adaptor::{
        get_rating_adaptor, get_ratings_analyses_adaptor, get_ratings_analysis_adaptor,
        get_ratings_city_adaptor, get_ratings_summaries_adaptor, patch_ratings_analysis_adaptor,
        post_ratings_analysis_adaptor,
    },
    BNAComponent,
};
use crate::{Context, ExecutionError};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use effortless::api::PaginationParameters;
use entity::wrappers::bna_pipeline::{BNAPipelinePatch, BNAPipelinePost};
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

pub fn routes() -> Router {
    Router::new()
        .route("/ratings", get(get_ratings))
        .route("/ratings/:rating_id", get(get_rating))
        .route("/ratings/:rating_id/city", get(get_ratings_city))
        .route(
            "/ratings/analyses",
            get(get_ratings_analyses).post(post_ratings_analysis),
        )
        .route(
            "/ratings/analyses/:analysis_id",
            get(get_ratings_analysis).patch(patch_ratings_analysis),
        )
}

async fn get_ratings(pagination: Option<Query<PaginationParameters>>) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    get_ratings_summaries_adaptor(pagination.page, pagination.page_size()).await
}

async fn get_rating(
    Path(rating_id): Path<Uuid>,
    component: Option<Query<BNAComponent>>,
    ctx: Context,
) -> Result<Json<Value>, ExecutionError> {
    let component = match component {
        Some(c) => {
            let Query(c) = c;
            Some(c)
        }
        None => None,
    };

    get_rating_adaptor(rating_id, component, ctx)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(|v| Json(json!(v)))
}

async fn get_ratings_city(
    Path(rating_id): Path<Uuid>,
    ctx: Context,
) -> Result<Json<Value>, ExecutionError> {
    get_ratings_city_adaptor(rating_id, ctx).await.map(Json)
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
    ctx: Context,
) -> Result<Json<Value>, ExecutionError> {
    get_ratings_analysis_adaptor(analysis_id, ctx)
        .await
        .map(Json)
}

async fn post_ratings_analysis(
    Json(bna_pipeline): Json<BNAPipelinePost>,
) -> Result<(StatusCode, Json<Value>), ExecutionError> {
    post_ratings_analysis_adaptor(bna_pipeline)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(|v| (StatusCode::CREATED, Json(v)))
}

async fn patch_ratings_analysis(
    Path(analysis_id): Path<Uuid>,
    Json(bna_pipeline): Json<BNAPipelinePatch>,
) -> Result<Json<Value>, ExecutionError> {
    patch_ratings_analysis_adaptor(bna_pipeline, analysis_id)
        .await
        .map(Json)
}
