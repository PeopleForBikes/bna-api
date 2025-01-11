use super::adaptor::{
    get_pipelines_bna_adaptor, get_pipelines_bnas_adaptor, patch_pipelines_bna_adaptor,
    post_pipelines_bna_adaptor,
};
use super::schema::{BnaPipeline, BnaPipelinePatch, BnaPipelinePost, BnaPipelines};
use crate::{
    core::resource::schema::{ErrorResponses, PaginationParams},
    Context, ExecutionError,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use effortless::api::PaginationParameters;
use entity::wrappers::bna_pipeline::{BNAPipelinePatch, BNAPipelinePost};
use serde_json::{json, Value};
use tracing::debug;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

const TAG: &str = "pipeline";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_pipelines_bna))
        .routes(routes!(get_pipelines_bnas))
        .routes(routes!(post_pipelines_bna))
        .routes(routes!(patch_pipelines_bna))
}

#[utoipa::path(
  get,
  path = "/pipelines/bna/{pipeline_id}",
  description = "Get the details of a specific BNA pipeline",
  tag = TAG,
  params(
    ("pipeline_id" = Uuid, Path, description = "Pipeline identifier")
  ),
  responses(
    (status = OK, description = "Fetches the details of a BNA pipeline", body = BnaPipeline),
    ErrorResponses,
  ))]
async fn get_pipelines_bna(
    Path(pipeline_id): Path<Uuid>,
    ctx: Context,
) -> Result<Json<BnaPipeline>, ExecutionError> {
    get_pipelines_bna_adaptor(pipeline_id, ctx)
        .await
        .map(BnaPipeline::from)
        .map(Json)
}

#[utoipa::path(
  get,
  path = "/pipelines/bna",
  description = "Get the details of all BNA pipelines",
  tag = TAG,
  params(
    PaginationParams,
  ),
  responses(
    (status = OK, description = "Fetches the details of all BNA pipelines", body = BnaPipelines),
    ErrorResponses,
  ))]
async fn get_pipelines_bnas(
    Query(pagination): Query<PaginationParameters>,
) -> Result<Json<Value>, ExecutionError> {
    get_pipelines_bnas_adaptor(pagination.page(), pagination.page_size())
        .await
        .map(|v| Json(json!(v.payload())))
}

#[utoipa::path(
  post,
  path = "/pipelines/bna",
  description = "Create a new BNA pipeline",
  tag = TAG,
  request_body = BnaPipelinePost,
  responses(
    (status = CREATED, description = "Creates a new city submission", body = BnaPipeline),
    ErrorResponses,
  ))]
async fn post_pipelines_bna(
    Json(bna_pipeline): Json<BNAPipelinePost>,
) -> Result<(StatusCode, Json<BnaPipeline>), ExecutionError> {
    post_pipelines_bna_adaptor(bna_pipeline)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(BnaPipeline::from)
        .map(|v| (StatusCode::CREATED, Json(v)))
}

#[utoipa::path(
  patch,
  path = "/pipelines/bna/{pipeline_id}",
  description = "Update the details of a specific BNA pipeline",
  tag = TAG,
  request_body = BnaPipelinePatch,
  params(
    ("pipeline_id" = Uuid, Path, description = "Pipeline identifier")
  ),
  responses(
    (status = OK, description = "Updates the details of a BNA pipeline", body = BnaPipeline),
    ErrorResponses,
  ))]
async fn patch_pipelines_bna(
    Path(analysis_id): Path<Uuid>,
    Json(bna_pipeline): Json<BNAPipelinePatch>,
) -> Result<Json<BnaPipeline>, ExecutionError> {
    patch_pipelines_bna_adaptor(bna_pipeline, analysis_id)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(BnaPipeline::from)
        .map(Json)
}
