use super::db::{fetch_bna_pipeline, fetch_bna_pipelines};
use crate::{Context, ExecutionError};
use entity::{
    bna_pipeline,
    wrappers::bna_pipeline::{BNAPipelinePatch, BNAPipelinePost},
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, IntoActiveModel, SqlErr};
use tracing::info;
use uuid::Uuid;

pub async fn get_pipelines_bna_adaptor(
    db: &DatabaseConnection,
    pipeline_id: Uuid,
    ctx: Context,
) -> Result<entity::bna_pipeline::Model, ExecutionError> {
    // Fetch the model.
    let model = fetch_bna_pipeline(db, pipeline_id).await?;
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a pipeline with the ID {pipeline_id}"),
        )),
    }
}

pub async fn get_pipelines_bnas_adaptor(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<bna_pipeline::Model>), ExecutionError> {
    // Fetch a page of pipelines.
    Ok(fetch_bna_pipelines(db, page, page_size).await?)
}

pub async fn post_pipelines_bna_adaptor(
    db: &DatabaseConnection,
    bna_pipeline: BNAPipelinePost,
    ctx: Context,
) -> Result<entity::bna_pipeline::Model, ExecutionError> {
    // Turn the post model into an active model.
    let state_machine_id = bna_pipeline.state_machine_id;
    let active_model = bna_pipeline.into_active_model();

    // And insert a new entry.
    info!(
        "inserting Brokenspoke pipeline into database: {:?}",
        active_model
    );
    match active_model.insert(db).await {
        Ok(model) => Ok(model),
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            Err(ExecutionError::Conflict(
                ctx.request_id(),
                ctx.source(),
                format!("the pipeline {state_machine_id} already exists"),
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn patch_pipelines_bna_adaptor(
    db: &DatabaseConnection,
    bna_pipeline: BNAPipelinePatch,
    analysis_id: Uuid,
    ctx: Context,
) -> Result<entity::bna_pipeline::Model, ExecutionError> {
    // Turn the patch model into an active model.
    let mut active_model = bna_pipeline.into_active_model();
    active_model.state_machine_id = ActiveValue::Unchanged(analysis_id);

    // Update the entry. An update without any change only looks the entry up,
    // and reports a missing entry as not found instead of not updated.
    match active_model.update(db).await {
        Ok(model) => Ok(model),
        Err(DbErr::RecordNotUpdated | DbErr::RecordNotFound(_)) => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a pipeline with the ID {analysis_id}"),
        )),
        Err(e) => Err(e.into()),
    }
}
