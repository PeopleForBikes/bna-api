use super::ExecutionError;
use crate::{
    cities::db::{fetch_cities, fetch_city, fetch_city_censuses},
    database_connect, PageFlow, Paginatron,
};
use serde_json::{json, Value};

pub async fn map_city(country: &str, region: &str, name: &str) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the city model.
    let model = fetch_city(&db, country, region, name).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            String::new(),
            format!("cannot find a city matching {country}, {region}, {name}"),
        )),
    }
}

pub async fn map_cities(page: u64, page_size: u64) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of cities.
    let (total_items, body) = fetch_cities(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(body),
    ))
}

pub async fn map_city_censuses(
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of city censuses.
    let (total_items, body) =
        fetch_city_censuses(&db, country, region, name, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(body),
    ))
}
