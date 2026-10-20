use super::db::{
    fetch_cities, fetch_cities_ratings, fetch_cities_submission, fetch_cities_submissions,
    fetch_cities_with_latest_summary, fetch_city, fetch_country, fetch_state_region_crosswalk,
    fetch_top_cities,
};
use crate::{core::resource::schema::OrderDirection, Context, ExecutionError};
use entity::{
    city, summary,
    wrappers::{
        city::CityPost,
        submission::{SubmissionPatch, SubmissionPost},
    },
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, IntoActiveModel, SqlErr};
use tracing::info;
use uuid::Uuid;

pub async fn get_city_adaptor(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
    ctx: Context,
) -> Result<city::Model, ExecutionError> {
    // Fetch the city model.
    let model = match fetch_city(db, country, region, name).await {
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
    db: &DatabaseConnection,
    sort_direction: OrderDirection,
    sort_by: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::city::Model>), ExecutionError> {
    // Fetch a page of cities.
    Ok(fetch_cities(db, sort_direction, sort_by, page, page_size).await?)
}

pub async fn get_cities_ratings_adaptor(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
    ctx: Context,
) -> Result<(city::Model, u64, Vec<summary::Model>), ExecutionError> {
    // Ensure the city exists: a page past the last one has no ratings, but the
    // city is still found.
    let city = get_city_adaptor(db, country, region, name, ctx).await?;

    // Fetch a page of city ratings. A city without ratings has a single row
    // without summary.
    let (total_items, models) =
        fetch_cities_ratings(db, country, region, name, page, page_size).await?;
    let ratings = models.into_iter().filter_map(|(_, s)| s).collect();

    Ok((city, total_items, ratings))
}

pub async fn post_cities_adaptor(
    db: &DatabaseConnection,
    city: CityPost,
    ctx: Context,
) -> Result<city::Model, ExecutionError> {
    // Ensure the country is a valid one.
    if fetch_country(db, &city.country).await?.is_none() {
        return Err(ExecutionError::UncoveredCountry(city.country));
    }

    // extract some fields from the post model.
    let country = city.country.clone();
    let state = city.state.clone();
    let name = city.name.clone();
    let region = city.region.clone();

    // Turn the post model into an active model.
    let mut active_model: entity::city::ActiveModel = city.into_active_model();

    // Assign a city_id.
    active_model.id = ActiveValue::Set(Uuid::new_v4());

    // If the country is the United States, set the BNA region. Otherwise, if
    // the region is not set, ensure it is equal to the country.
    if country.to_lowercase().eq("united states") {
        match fetch_state_region_crosswalk(db, &state).await? {
            Some(model) => {
                active_model.region = ActiveValue::Set(Some(model.region));
            }
            None => return Err(ExecutionError::InvalidUSState(state)),
        }
    } else if region.is_none() {
        active_model.region = ActiveValue::Set(Some(country.clone()));
    }

    // And insert a new entry.
    match active_model.insert(db).await {
        Ok(model) => Ok(model),
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            Err(ExecutionError::Conflict(
                ctx.request_id(),
                ctx.source(),
                format!("the city {name}, {state}, {country} already exists"),
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn get_cities_submission_adaptor(
    db: &DatabaseConnection,
    submission_id: i32,
    status: Option<String>,
    ctx: Context,
) -> Result<entity::submission::Model, ExecutionError> {
    // Fetch the model.
    let status_str = status.clone().unwrap_or("any".to_string());
    let model = fetch_cities_submission(db, submission_id, status).await?;
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
    db: &DatabaseConnection,
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::submission::Model>), ExecutionError> {
    // Fetch the model.
    Ok(fetch_cities_submissions(db, status, page, page_size).await?)
}

pub async fn patch_cities_submission_adaptor(
    db: &DatabaseConnection,
    submission_id: i32,
    submission: SubmissionPatch,
    ctx: Context,
) -> Result<entity::submission::Model, ExecutionError> {
    // Turn the wrapper into an active model.
    let mut active_model = submission.into_active_model();
    active_model.id = ActiveValue::Unchanged(submission_id);

    // An update without any change only looks the entry up, and reports a
    // missing entry as not found instead of not updated.
    match active_model.update(db).await {
        Ok(model) => Ok(model),
        Err(DbErr::RecordNotUpdated | DbErr::RecordNotFound(_)) => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find submission with id {submission_id}"),
        )),
        Err(e) => Err(e.into()),
    }
}

pub async fn post_cities_submission_adaptor(
    db: &DatabaseConnection,
    submission: SubmissionPost,
) -> Result<entity::submission::Model, ExecutionError> {
    // note(rgreinho): Leaving it commented out here because this check is still under
    // discussion.
    // Ensure the country is a valid one.
    // if fetch_country(&db, &submission.country).await?.is_none() {
    //     return Err(ExecutionError::UncoveredCountry(submission.country));
    // }

    // Turn the post model into an active model.
    let country = submission.country.clone();
    let active_model: entity::submission::ActiveModel = submission.into_active_model();

    // And insert a new entry. The country is the only foreign key set by the
    // client.
    match active_model.insert(db).await {
        Ok(model) => Ok(model),
        Err(e) if matches!(e.sql_err(), Some(SqlErr::ForeignKeyConstraintViolation(_))) => {
            Err(ExecutionError::UncoveredCountry(country))
        }
        Err(e) => Err(e.into()),
    }
}

pub(crate) async fn get_top_cities_adaptor(
    db: &DatabaseConnection,
    year: i32,
    count: u64,
    ctx: Context,
) -> Result<Vec<(city::Model, summary::Model)>, ExecutionError> {
    // Fetch the top cities and their associated summaries.
    let model = match fetch_top_cities(db, year, count).await {
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

pub async fn get_cities_latest_summary_adaptor(
    db: &DatabaseConnection,
    sort_direction: OrderDirection,
    sort_by: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<(city::Model, summary::Model)>), ExecutionError> {
    // Fetch all cities with their latest summary.
    Ok(fetch_cities_with_latest_summary(db, sort_direction, sort_by, page, page_size).await?)
}
