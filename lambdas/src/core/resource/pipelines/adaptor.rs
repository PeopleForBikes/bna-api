use super::db::{fetch_bna_pipeline, fetch_bna_pipelines};
use crate::{database_connect, Context, ExecutionError, PageFlow, Paginatron};
use entity::{
    bna_pipeline,
    wrappers::bna_pipeline::{BNAPipelinePatch, BNAPipelinePost},
};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use tracing::info;
use uuid::Uuid;

pub async fn get_pipelines_bna_adaptor(
    pipeline_id: Uuid,
    ctx: Context,
) -> Result<entity::bna_pipeline::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch the model.
    let model = fetch_bna_pipeline(&db, pipeline_id).await?;
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
    page: u64,
    page_size: u64,
) -> Result<PageFlow<Vec<bna_pipeline::Model>>, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_bna_pipelines(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        models,
    ))
}

pub async fn post_pipelines_bna_adaptor(
    bna_pipeline: BNAPipelinePost,
) -> Result<entity::bna_pipeline::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Turn the post model into an active model.
    let active_model = bna_pipeline.into_active_model();

    // And insert a new entry.
    info!(
        "inserting Brokenspoke pipeline into database: {:?}",
        active_model
    );
    let model = active_model.insert(&db).await?;
    Ok(model)
}

pub async fn patch_pipelines_bna_adaptor(
    bna_pipeline: BNAPipelinePatch,
    analysis_id: Uuid,
) -> Result<entity::bna_pipeline::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Turn the patch model into an active model.
    let mut active_model = bna_pipeline.into_active_model();
    active_model.state_machine_id = ActiveValue::Unchanged(analysis_id);

    // Update the entry.
    let model = active_model.update(&db).await?;
    Ok(model)
}
