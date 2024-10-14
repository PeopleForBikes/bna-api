use crate::{
    cities::ExecutionError, database_connect, ratings::db::fetch_ratings_summaries, PageFlow,
    Paginatron,
};
use serde_json::{json, Value};
use uuid::Uuid;

use super::db::{
    fetch_ratings_analyses, fetch_ratings_analysis, fetch_ratings_city, fetch_ratings_summary,
};

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

pub async fn get_ratings_summary_adaptor(rating_id: Uuid) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_summary(&db, rating_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            String::new(),
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

pub async fn get_ratings_analysis_adaptor(analysis_id: Uuid) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_analysis(&db, analysis_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            String::new(),
            format!("cannot find a rating with the ID {analysis_id}"),
        )),
    }
}

pub async fn get_ratings_city_adaptor(rating_id: Uuid) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_city(&db, rating_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            String::new(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}
