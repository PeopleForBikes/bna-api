use super::{
    db::{fetch_cities_ratings, fetch_cities_submission, fetch_cities_submissions},
    ExecutionError,
};
use crate::{
    cities::db::{
        fetch_cities, fetch_cities_censuses, fetch_city, fetch_country,
        fetch_state_region_crosswalk,
    },
    database_connect, PageFlow, Paginatron,
};
use entity::wrappers::city::CityPost;
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn get_city_adaptor(
    country: &str,
    region: &str,
    name: &str,
) -> Result<Value, ExecutionError> {
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

pub async fn get_cities_adaptor(page: u64, page_size: u64) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of cities.
    let (total_items, models) = fetch_cities(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_cities_censuses_adaptor(
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of city censuses.
    let (total_items, models) =
        fetch_cities_censuses(&db, country, region, name, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_cities_ratings_adaptor(
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of city censuses.
    let (total_items, models) =
        fetch_cities_ratings(&db, country, region, name, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn post_cities_adaptor(city: CityPost) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Ensure the country is a valid one.
    if fetch_country(&db, &city.country).await?.is_none() {
        return Err(ExecutionError::UncoveredCountry(city.country));
    }

    // extract some fields from the post model.
    let country = city.country.clone();
    let state = city.state.clone();
    let region = city.region.clone();

    // Turn the post model into an active model.
    let mut active_city: entity::city::ActiveModel = city.into_active_model();

    // Assign a city_id.
    active_city.id = ActiveValue::Set(Uuid::new_v4());

    // If the country is the United States, set the BNA region.
    if country.to_lowercase().eq("united states") {
        match fetch_state_region_crosswalk(&db, &state).await? {
            Some(model) => {
                active_city.region = ActiveValue::Set(Some(model.region));
            }
            None => return Err(ExecutionError::InvalidUSState(state)),
        }
    }

    // If the region is not set, ensure it is equal to the country.
    if region.is_none() {
        active_city.region = ActiveValue::Set(Some(country));
    }

    // And insert a new entry.
    // info!("inserting City into database: {:?}", active_city);
    let model = active_city.insert(&db).await?;
    Ok(json!(model))
}

pub async fn get_cities_submission_adaptor(
    submission_id: i32,
    status: Option<String>,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let status_str = status.clone().unwrap_or("any".to_string());
    let model = fetch_cities_submission(&db, submission_id, status).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            String::new(),
            format!("cannot find submission with id {submission_id} and {status_str} status"),
        )),
    }
}

pub async fn get_cities_submissions_adaptor(
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let (total_items, models) = fetch_cities_submissions(&db, status, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}
