use super::adaptor::{
    get_pipelines_bna_adaptor, get_pipelines_bnas_adaptor, patch_pipelines_bna_adaptor,
    post_pipelines_bna_adaptor,
};
use super::schema::{BnaPipeline, BnaPipelinePatch, BnaPipelinePost, BnaPipelines};
use crate::core::extract::{ApiJson, ApiPath, ApiQuery, PageUrl};
use crate::database_connect_or_init;
use crate::{
    core::resource::schema::{
        BadRequestResponse, ConflictResponse, NotFoundResponse, PaginationParameters,
    },
    Context, ExecutionError, PageFlow, Paginatron,
};
use axum::{http::StatusCode, Json};
use entity::wrappers::bna_pipeline::{BNAPipelinePatch, BNAPipelinePost};
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
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn get_pipelines_bna(
    ApiPath(pipeline_id): ApiPath<Uuid>,
    ctx: Context,
) -> Result<Json<BnaPipeline>, ExecutionError> {
    let db = database_connect_or_init().await?;
    get_pipelines_bna_adaptor(db, pipeline_id, ctx)
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
    PaginationParameters,
  ),
  responses(
    (status = OK, description = "Fetches the details of all BNA pipelines", body = BnaPipelines),
    BadRequestResponse,
  ))]
async fn get_pipelines_bnas(
    ApiQuery(pagination): ApiQuery<PaginationParameters>,
    PageUrl(url): PageUrl,
) -> Result<PageFlow<Vec<entity::bna_pipeline::Model>>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let (total_items, models) =
        get_pipelines_bnas_adaptor(db, pagination.page(), pagination.page_size()).await?;
    Ok(PageFlow::new(
        Paginatron::new(url, total_items, pagination.page(), pagination.page_size()),
        models,
    ))
}

#[utoipa::path(
  post,
  path = "/pipelines/bna",
  description = "Create a new BNA pipeline",
  tag = TAG,
  request_body = BnaPipelinePost,
  responses(
    (status = CREATED, description = "Creates a new city submission", body = BnaPipeline),
    BadRequestResponse,
    ConflictResponse,
  ))]
async fn post_pipelines_bna(
    ctx: Context,
    ApiJson(bna_pipeline): ApiJson<BNAPipelinePost>,
) -> Result<(StatusCode, Json<BnaPipeline>), ExecutionError> {
    let db = database_connect_or_init().await?;
    post_pipelines_bna_adaptor(db, bna_pipeline, ctx)
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
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn patch_pipelines_bna(
    ctx: Context,
    ApiPath(analysis_id): ApiPath<Uuid>,
    ApiJson(bna_pipeline): ApiJson<BNAPipelinePatch>,
) -> Result<Json<BnaPipeline>, ExecutionError> {
    let db = database_connect_or_init().await?;
    patch_pipelines_bna_adaptor(db, bna_pipeline, analysis_id, ctx)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(BnaPipeline::from)
        .map(Json)
}
