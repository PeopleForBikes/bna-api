use super::db::{
    fetch_cities, fetch_cities_ratings, fetch_cities_submission, fetch_cities_submissions,
    fetch_city, fetch_country, fetch_state_region_crosswalk, fetch_top_cities,
};
use crate::{database_connect, Context, ExecutionError};
use entity::{
    city, summary,
    wrappers::{
        city::CityPost,
        submission::{SubmissionPatch, SubmissionPost},
    },
};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serde_json::{json, Value};
use tracing::info;
use uuid::Uuid;

pub async fn get_city_adaptor(
    country: &str,
    region: &str,
    name: &str,
    ctx: Context,
) -> Result<city::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch the city model.
    let model = match fetch_city(&db, country, region, name).await {
        Ok(model) => model,
        Err(e) => {
            info!("{e:?}");
            return Err(e.into());
        }
    };
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a city matching {country}, {region}, {name}"),
        )),
    }
}

pub async fn get_cities_adaptor(
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::city::Model>), ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch a page of cities.
    Ok(fetch_cities(&db, page, page_size).await?)
}

pub async fn get_cities_ratings_adaptor(
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
    ctx: Context,
) -> Result<(u64, Vec<(city::Model, Option<summary::Model>)>), ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch a page of city censuses.
    let (total_items, models) =
        fetch_cities_ratings(&db, country, region, name, page, page_size).await?;

    // If there is no model, the city resource does not exist.
    if models.is_empty() {
        return Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a city matching {country}, {region}, {name}"),
        ));
    }

    Ok((total_items, models))
}

pub async fn post_cities_adaptor(city: CityPost) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Ensure the country is a valid one.
    if fetch_country(&db, &city.country).await?.is_none() {
        return Err(ExecutionError::UncoveredCountry(city.country));
    }

    // extract some fields from the post model.
    let country = city.country.clone();
    let state = city.state.clone();
    let region = city.region.clone();

    // Turn the post model into an active model.
    let mut active_model: entity::city::ActiveModel = city.into_active_model();

    // Assign a city_id.
    active_model.id = ActiveValue::Set(Uuid::new_v4());

    // If the country is the United States, set the BNA region.
    if country.to_lowercase().eq("united states") {
        match fetch_state_region_crosswalk(&db, &state).await? {
            Some(model) => {
                active_model.region = ActiveValue::Set(Some(model.region));
            }
            None => return Err(ExecutionError::InvalidUSState(state)),
        }
    }

    // If the region is not set, ensure it is equal to the country.
    if region.is_none() {
        active_model.region = ActiveValue::Set(Some(country));
    }

    // And insert a new entry.
    let model = active_model.insert(&db).await?;
    Ok(json!(model))
}

pub async fn get_cities_submission_adaptor(
    submission_id: i32,
    status: Option<String>,
    ctx: Context,
) -> Result<entity::submission::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch the model.
    let status_str = status.clone().unwrap_or("any".to_string());
    let model = fetch_cities_submission(&db, submission_id, status).await?;
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find submission with id {submission_id} and {status_str} status"),
        )),
    }
}

pub async fn get_cities_submissions_adaptor(
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::submission::Model>), ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch the model.
    Ok(fetch_cities_submissions(&db, status, page, page_size).await?)
}

pub async fn patch_cities_submission_adaptor(
    submission_id: i32,
    submission: SubmissionPatch,
) -> Result<entity::submission::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Turn the wrapper into an active model.
    let mut active_model = submission.into_active_model();
    active_model.id = ActiveValue::Unchanged(submission_id);

    let model = active_model.update(&db).await?;
    Ok(model)
}

pub async fn post_cities_submission_adaptor(
    submission: SubmissionPost,
) -> Result<entity::submission::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // note(rgreinho): Leaving it commented out here because this check is still under
    // discussion.
    // Ensure the country is a valid one.
    // if fetch_country(&db, &submission.country).await?.is_none() {
    //     return Err(ExecutionError::UncoveredCountry(submission.country));
    // }

    // Turn the post model into an active model.
    let mut active_model: entity::submission::ActiveModel = submission.into_active_model();

    // Add the status.
    active_model.status = ActiveValue::Set("Pending".to_string());

    // And insert a new entry.
    let model = active_model.insert(&db).await?;
    Ok(model)
}

pub(crate) async fn get_top_cities_adaptor(
    year: i32,
    count: i32,
    ctx: Context,
) -> Result<Vec<(city::Model, summary::Model)>, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch the top cities and their associated summaries.
    let model = match fetch_top_cities(&db, year, count).await {
        Ok(model) => model,
        Err(e) => {
            info!("{e:?}");
            return Err(e.into());
        }
    };
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot fetch the {count} top cities for the year {year}"),
        )),
    }
}
