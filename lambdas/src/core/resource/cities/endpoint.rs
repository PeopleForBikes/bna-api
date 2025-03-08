use super::{
    adaptor::{
        get_cities_adaptor, get_cities_ratings_adaptor, get_cities_submission_adaptor,
        get_cities_submissions_adaptor, get_city_adaptor, patch_cities_submission_adaptor,
        post_cities_adaptor, post_cities_submission_adaptor,
    },
    schema::{
        Cities, CityPost, CityRatings, RatingSummary, Submission, SubmissionPatch, SubmissionPost,
        Submissions,
    },
};
use crate::{
    core::resource::{
        cities::{schema::CityParams, CitiesPathParameters},
        schema::{City, ErrorResponses, PaginationParams},
    },
    Context, ExecutionError, PageFlow, Paginatron,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use axum_extra::extract::OptionalQuery;
use effortless::api::PaginationParameters;
use entity::wrappers::{city, submission};
use serde::{self, Deserialize};
use serde_json::Value;
use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "city";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_city))
        .routes(routes!(post_city))
        .routes(routes!(get_cities))
        .routes(routes!(get_city_ratings))
        .routes(routes!(get_cities_submission))
        .routes(routes!(post_cities_submission))
        .routes(routes!(patch_cities_submission))
        .routes(routes!(get_cities_submissions))
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
    ErrorResponses,
  ))]
async fn get_city(
    Path(params): Path<CitiesPathParameters>,
    ctx: Context,
) -> Result<Json<City>, ExecutionError> {
    get_city_adaptor(&params.country, &params.region, &params.name, ctx)
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
    PaginationParams,
  ),
  responses(
    (status = OK, description = "Fetches cities", body = Cities),
  ))]
async fn get_cities(
    Query(pagination): Query<PaginationParameters>,
) -> Result<PageFlow<Cities>, ExecutionError> {
    // let Query(pagination) = pagination;
    let payload = get_cities_adaptor(pagination.page(), pagination.page_size()).await?;
    Ok(PageFlow::new(
        Paginatron::new(None, payload.0, pagination.page(), pagination.page_size()),
        payload.1.into(),
    ))
}

#[utoipa::path(
  get,
  path = "/cities/{country}/{region}/{name}/ratings",
  description = "Get the details of a specific city with all the analysis that were performed against it.",
  tag = TAG,
  params(
    CityParams,
    PaginationParams,
  ),
  responses(
    (status = OK, description = "Fetches city ratings", body = CityRatings),
    ErrorResponses,
  ))]
async fn get_city_ratings(
    Path(params): Path<CitiesPathParameters>,
    Query(pagination): Query<PaginationParameters>,
    ctx: Context,
) -> Result<PageFlow<CityRatings>, ExecutionError> {
    let city_ratings = get_cities_ratings_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page(),
        pagination.page_size(),
        ctx,
    )
    .await?;
    let city = city_ratings.1.first().unwrap().0.clone();
    let ratings = city_ratings
        .1
        .iter()
        .filter_map(|e| e.1.clone())
        .map(RatingSummary::from)
        .collect::<Vec<RatingSummary>>();
    let payload = CityRatings {
        city: city.into(),
        ratings,
    };
    Ok(PageFlow::new(
        Paginatron::new(
            None,
            city_ratings.0,
            pagination.page(),
            pagination.page_size(),
        ),
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
    ErrorResponses,
  ))]
async fn post_city(Json(city): Json<city::CityPost>) -> Result<Json<Value>, ExecutionError> {
    post_cities_adaptor(city).await.map(Json)
}

#[utoipa::path(
  get,
  path = "/cities/submissions/{submission_id}",
  description = "Get the details of a specific sumission.",
  tag = TAG,
  params(
    ("submission_id" = i32, Path, description = "Submission identifier", example = "1"),
    ("status" = Option<str>, Query, description = "Filter for the submission status", example = "Pending"),
  ),
  responses(
    (status = OK, description = "Fetches a submission", body = Submission),
    ErrorResponses,
  ))]
async fn get_cities_submission(
    Path(submission_id): Path<i32>,
    OptionalQuery(status): OptionalQuery<String>,
    ctx: Context,
) -> Result<Json<Submission>, ExecutionError> {
    get_cities_submission_adaptor(submission_id, status, ctx)
        .await
        .map(Submission::from)
        .map(Json)
}

#[derive(Deserialize)]
struct SubmissionParameters {
    pub status: Option<String>,
}

#[utoipa::path(
    get,
    path = "/cities/submissions",
    description = "Get the submissions details.",
    tag = TAG,
    params(
      ("status" = Option<str>, Query, description = "Filter for the submission status", example = "Pending"),
      PaginationParams,
    ),
    responses(
      (status = OK, description = "Fetches submissions", body = Submissions),
      ErrorResponses,
    ))]
async fn get_cities_submissions(
    Query(submission_params): Query<SubmissionParameters>,
    Query(pagination): Query<PaginationParameters>,
) -> Result<PageFlow<Submissions>, ExecutionError> {
    let cities_submissions = get_cities_submissions_adaptor(
        submission_params.status,
        pagination.page(),
        pagination.page_size(),
    )
    .await?;
    // .map(|v| Json(json!(v.payload())))
    let submissions = cities_submissions
        .1
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();
    let payload = Submissions(submissions);
    Ok(PageFlow::new(
        Paginatron::new(
            None,
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
    ErrorResponses,
  ))]
async fn post_cities_submission(
    Json(submission): Json<submission::SubmissionPost>,
) -> Result<(StatusCode, Json<Submission>), ExecutionError> {
    post_cities_submission_adaptor(submission)
        .await
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
    ErrorResponses,
  ))]
async fn patch_cities_submission(
    Query(submission_id): Query<i32>,
    Json(submission): Json<submission::SubmissionPatch>,
) -> Result<Json<Submission>, ExecutionError> {
    patch_cities_submission_adaptor(submission_id, submission)
        .await
        .map(Submission::from)
        .map(Json)
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
