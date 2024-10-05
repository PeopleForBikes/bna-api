use super::db::{
    fetch_ratings_analyses, fetch_ratings_analysis, fetch_ratings_city, fetch_ratings_summaries,
    fetch_ratings_summary,
};
use crate::{database_connect, Context, ExecutionError, PageFlow, Paginatron};
use entity::wrappers::bna_pipeline::{BNAPipelinePatch, BNAPipelinePost};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serde_json::{json, Value};
use tracing::info;
use uuid::Uuid;

pub async fn get_ratings_summaries_adaptor(
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_ratings_summaries(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_ratings_summary_adaptor(
    rating_id: Uuid,
    ctx: Context,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_summary(&db, rating_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub async fn get_ratings_adaptor(page: u64, page_size: u64) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_ratings_summaries(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_ratings_analyses_adaptor(
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_ratings_analyses(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_ratings_analysis_adaptor(
    analysis_id: Uuid,
    ctx: Context,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_analysis(&db, analysis_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {analysis_id}"),
        )),
    }
}

pub async fn get_ratings_city_adaptor(
    rating_id: Uuid,
    ctx: Context,
) -> Result<Value, ExecutionError> {
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_city(&db, rating_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub async fn post_ratings_analysis_adaptor(
    bna_pipeline: BNAPipelinePost,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Turn the post model into an active model.
    let active_model = bna_pipeline.into_active_model();

    // And insert a new entry.
    info!(
        "inserting Brokenspoke pipeline into database: {:?}",
        active_model
    );
    let model = active_model.insert(&db).await?;
    Ok(json!(model))
}

pub async fn patch_ratings_analysis_adaptor(
    bna_pipeline: BNAPipelinePatch,
    analysis_id: Uuid,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Turn the patch model into an active model.
    let mut active_model = bna_pipeline.into_active_model();
    active_model.state_machine_id = ActiveValue::Unchanged(analysis_id);

    // Update the entry.
    let model = active_model.update(&db).await?;
    Ok(json!(model))
}
