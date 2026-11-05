use super::{
    adaptor::{
        get_cities_adaptor, get_cities_latest_summary_adaptor, get_cities_ratings_adaptor,
        get_cities_submission_adaptor, get_cities_submissions_adaptor, get_city_adaptor,
        patch_cities_submission_adaptor, post_cities_adaptor, post_cities_submission_adaptor,
    },
    schema::{
        ApprovalStatus, Cities, CitiesWithSummary, CityListParameters, CityPost, CityRatings,
        RatingSummary, Submission, SubmissionPatch, SubmissionPost, Submissions,
    },
};
use crate::core::extract::{ApiJson, ApiPath, ApiQuery, PageUrl};
use crate::{
    core::resource::{
        cities::{
            adaptor::get_top_cities_adaptor,
            schema::{CityParams, CityWithSummary},
            CitiesPathParameters,
        },
        schema::{
            deserialize_bna_year, BadRequestResponse, City, ConflictResponse, NotFoundResponse,
            PaginationParameters,
        },
    },
    database_connect_or_init, Context, ExecutionError, PageFlow, Paginatron,
};
use axum::{http::StatusCode, Json};
use entity::wrappers::{city, submission};
use serde::{self, Deserialize};
use tracing::debug;
use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "city";
const TOP_CITIES_MAX_COUNT: u64 = 100;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_city))
        .routes(routes!(post_city))
        .routes(routes!(get_cities))
        .routes(routes!(get_cities_latest_summary))
        .routes(routes!(get_city_ratings))
        .routes(routes!(get_cities_submission))
        .routes(routes!(post_cities_submission))
        .routes(routes!(patch_cities_submission))
        .routes(routes!(get_cities_submissions))
        .routes(routes!(get_top_cities))
}

#[utoipa::path(
  get,
  path = "/cities/{country}/{region}/{name}",
  description = "Get the details of a specific city where an BNA analysis was computed.",
  tag = TAG,
  params(
    CityParams,
  ),
  responses(
    (status = OK, description = "Fetches a city", body = City),
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn get_city(
    ApiPath(params): ApiPath<CitiesPathParameters>,
    ctx: Context,
) -> Result<Json<City>, ExecutionError> {
    let db = database_connect_or_init().await?;
    get_city_adaptor(db, &params.country, &params.region, &params.name, ctx)
        .await
        .map(City::from)
        .map(Json)
}

#[axum::debug_handler]
#[utoipa::path(
  get,
  path = "/cities",
  description = "Get the details of all cities where an BNA analysis was performed.",
  tag = TAG,
  params(
    CityListParameters,
  ),
  responses(
    (status = OK, description = "Fetches cities", body = Cities),
    BadRequestResponse,
  ))]
async fn get_cities(
    ApiQuery(list): ApiQuery<CityListParameters>,
    PageUrl(url): PageUrl,
) -> Result<PageFlow<Cities>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let payload = get_cities_adaptor(
        db,
        list.order_direction(),
        &list.sort_by(),
        list.page(),
        list.page_size(),
    )
    .await?;
    Ok(PageFlow::new(
        Paginatron::new(url, payload.0, list.page(), list.page_size()),
        payload.1.into(),
    ))
}

#[utoipa::path(
  get,
  path = "/cities/summary/latest",
  description = "Get all cities with their latest summary.",
  tag = TAG,
  params(
    CityListParameters,
  ),
  responses(
    (status = OK, description = "Fetches cities with their latest summary", body = CitiesWithSummary),
    BadRequestResponse,
  ))]
async fn get_cities_latest_summary(
    ApiQuery(list): ApiQuery<CityListParameters>,
    PageUrl(url): PageUrl,
) -> Result<PageFlow<CitiesWithSummary>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let payload = get_cities_latest_summary_adaptor(
        db,
        list.order_direction(),
        &list.sort_by(),
        list.page(),
        list.page_size(),
    )
    .await
    .map_err(|e| {
        debug!("{e}");
        e
    })?;

    let cities_with_summaries = payload
        .1
        .iter()
        .map(|(c, s)| CityWithSummary {
            city: c.clone().into(),
            summary: s.clone().into(),
        })
        .collect::<Vec<CityWithSummary>>();

    Ok(PageFlow::new(
        Paginatron::new(url, payload.0, list.page(), list.page_size()),
        CitiesWithSummary(cities_with_summaries),
    ))
}

#[utoipa::path(
  get,
  path = "/cities/{country}/{region}/{name}/ratings",
  description = "Get the details of a specific city with all the analysis that were performed against it.",
  tag = TAG,
  params(
    CityParams,
    PaginationParameters,
  ),
  responses(
    (status = OK, description = "Fetches city ratings", body = CityRatings),
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn get_city_ratings(
    ApiPath(params): ApiPath<CitiesPathParameters>,
    ApiQuery(pagination): ApiQuery<PaginationParameters>,
    PageUrl(url): PageUrl,
    ctx: Context,
) -> Result<PageFlow<CityRatings>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let city_ratings = get_cities_ratings_adaptor(
        db,
        &params.country,
        &params.region,
        &params.name,
        pagination.page(),
        pagination.page_size(),
        ctx,
    )
    .await?;
    let (city, total_items, ratings) = city_ratings;
    let payload = CityRatings {
        city: city.into(),
        ratings: ratings.into_iter().map(RatingSummary::from).collect(),
    };
    Ok(PageFlow::new(
        Paginatron::new(url, total_items, pagination.page(), pagination.page_size()),
        payload,
    ))
}

#[utoipa::path(
  post,
  path = "/cities",
  description = "Create a new city.",
  tag = TAG,
  request_body = CityPost,
  responses(
    (status = CREATED, description = "Creates a new city", body = City),
    BadRequestResponse,
    ConflictResponse,
  ))]
async fn post_city(
    ctx: Context,
    ApiJson(city): ApiJson<city::CityPost>,
) -> Result<(StatusCode, Json<City>), ExecutionError> {
    let db = database_connect_or_init().await?;
    post_cities_adaptor(db, city, ctx)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(City::from)
        .map(|v| (StatusCode::CREATED, Json(v)))
}

#[utoipa::path(
  get,
  path = "/cities/submissions/{submission_id}",
  description = "Get the details of a specific sumission.",
  tag = TAG,
  params(
    ("submission_id" = i32, Path, description = "Submission identifier", example = "1"),
    ("status" = Option<ApprovalStatus>, Query, description = "Filter for the submission status", example = "Pending"),
  ),
  responses(
    (status = OK, description = "Fetches a submission", body = Submission),
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn get_cities_submission(
    ApiPath(submission_id): ApiPath<i32>,
    ApiQuery(submission_params): ApiQuery<SubmissionParameters>,
    ctx: Context,
) -> Result<Json<Submission>, ExecutionError> {
    let db = database_connect_or_init().await?;
    get_cities_submission_adaptor(
        db,
        submission_id,
        submission_params.status.map(|s| s.to_string()),
        ctx,
    )
    .await
    .map(Submission::from)
    .map(Json)
}

/// Path parameters of the top cities, within the documented bounds.
#[derive(Deserialize)]
struct TopCitiesPath {
    #[serde(deserialize_with = "deserialize_bna_year")]
    year: i32,
    #[serde(deserialize_with = "deserialize_top_cities_count")]
    count: u64,
}

/// Deserializes the number of top cities, between 1 and `TOP_CITIES_MAX_COUNT`.
fn deserialize_top_cities_count<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let count = u64::deserialize(deserializer)?;
    if (1..=TOP_CITIES_MAX_COUNT).contains(&count) {
        Ok(count)
    } else {
        Err(serde::de::Error::custom(format!(
            "the count must be between 1 and {TOP_CITIES_MAX_COUNT}"
        )))
    }
}

#[derive(Deserialize)]
struct SubmissionParameters {
    pub status: Option<ApprovalStatus>,
}

#[utoipa::path(
    get,
    path = "/cities/submissions",
    description = "Get the submissions details.",
    tag = TAG,
    params(
      ("status" = Option<ApprovalStatus>, Query, description = "Filter for the submission status", example = "Pending"),
      PaginationParameters,
    ),
    responses(
      (status = OK, description = "Fetches submissions", body = Submissions),
      BadRequestResponse,
    ))]
async fn get_cities_submissions(
    ApiQuery(submission_params): ApiQuery<SubmissionParameters>,
    ApiQuery(pagination): ApiQuery<PaginationParameters>,
    PageUrl(url): PageUrl,
) -> Result<PageFlow<Submissions>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let cities_submissions = get_cities_submissions_adaptor(
        db,
        submission_params.status.map(|s| s.to_string()),
        pagination.page(),
        pagination.page_size(),
    )
    .await?;
    let submissions = cities_submissions
        .1
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();
    let payload = Submissions(submissions);
    Ok(PageFlow::new(
        Paginatron::new(
            url,
            cities_submissions.0,
            pagination.page(),
            pagination.page_size(),
        ),
        payload,
    ))
}

#[utoipa::path(
  post,
  path = "/cities/submissions",
  description = "Create a new city submission.",
  tag = TAG,
  request_body = SubmissionPost,
  responses(
    (status = CREATED, description = "Creates a new city submission", body = Submission),
    BadRequestResponse,
  ))]
async fn post_cities_submission(
    ctx: Context,
    ApiJson(submission): ApiJson<submission::SubmissionPost>,
) -> Result<(StatusCode, Json<Submission>), ExecutionError> {
    let db = database_connect_or_init().await?;
    post_cities_submission_adaptor(db, submission, ctx)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(Submission::from)
        .map(|v| (StatusCode::CREATED, Json(v)))
}

#[utoipa::path(
  patch,
  path = "/cities/submissions/{submission_id}",
  description = "Update a city submission.",
  tag = TAG,
  params(
    ("submission_id" = i32, Path, description = "Submission identifier", example = "1"),
  ),
  request_body = SubmissionPatch,
  responses(
    (status = OK, description = "Updates a city submission", body = Submission),
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn patch_cities_submission(
    ApiPath(submission_id): ApiPath<i32>,
    ctx: Context,
    ApiJson(submission): ApiJson<submission::SubmissionPatch>,
) -> Result<Json<Submission>, ExecutionError> {
    let db = database_connect_or_init().await?;
    patch_cities_submission_adaptor(db, submission_id, submission, ctx)
        .await
        .map(Submission::from)
        .map(Json)
}

#[utoipa::path(
  get,
  path = "/cities/top/{year}/{count}",
  description = "Get the top N cities for a specific year.",
  tag = TAG,
  params(
    ("year" = i32, Path, description = "The year to collect the top cities for",  example = "2024", minimum = 2017, maximum = 2029),
    ("count" = u64, Path, description = "The number of top cities to collect",  example = "10", minimum = 1, maximum = 100),
  ),
  responses(
    (status = OK, description = "Fetches cities with their respective summary", body = CitiesWithSummary),
    BadRequestResponse,
    NotFoundResponse,
  ))]
async fn get_top_cities(
    ApiPath(TopCitiesPath { year, count }): ApiPath<TopCitiesPath>,
    ctx: Context,
) -> Result<Json<CitiesWithSummary>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let models = get_top_cities_adaptor(db, year, count, ctx).await?;

    let payload = models
        .iter()
        .map(|(c, s)| CityWithSummary {
            city: c.clone().into(),
            summary: s.clone().into(),
        })
        .collect::<Vec<CityWithSummary>>();

    Ok(Json(CitiesWithSummary(payload)))
}

#[cfg(test)]
mod tests {
    use axum::extract::Query;
    use lambda_http::http::Uri;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct ExampleParams {
        foo: Option<String>,
        // bar: u32,
    }

    #[test]
    fn query() {
        let uri: Uri = "http://example.com/path?bar=42".parse().unwrap();
        let result: Query<ExampleParams> = Query::try_from_uri(&uri).unwrap();
        // let result: Query<Option<String>> = Query::try_from_uri(&uri).unwrap();
        dbg!(&result);
        assert_eq!(result.foo, None);
        // assert_eq!(result.foo, String::from("hello"));
    }
}
