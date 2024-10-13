use crate::cities::{
    adaptor::{
        get_cities_adaptor, get_cities_censuses_adaptor, get_cities_ratings_adaptor,
        get_cities_submission_adaptor, get_cities_submissions_adaptor, get_city_adaptor,
        patch_cities_submission_adaptor, post_cities_adaptor, post_cities_submission_adaptor,
    },
    CitiesPathParameters, ExecutionError,
};
use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use axum_extra::extract::OptionalQuery;
use effortless::api::PaginationParameters;
use entity::wrappers::{
    city::CityPost,
    submission::{SubmissionPatch, SubmissionPost},
};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new()
        .route("/cities", get(get_cities).post(post_cities))
        .route("/cities/:country/:region/:name", get(get_city))
        .route(
            "/cities/:country/:region/:name/census",
            get(get_city_censuses),
        )
        .route(
            "/cities/:country/:region/:name/ratings",
            get(get_city_ratings),
        )
        .route(
            "/cities/submissions",
            get(get_cities_submissions).post(post_cities_submissions),
        )
        .route(
            "/cities/submissions/:submission_id",
            get(get_cities_submission).patch(patch_cities_submission),
        )
}

async fn get_city(Path(params): Path<CitiesPathParameters>) -> Result<Json<Value>, ExecutionError> {
    get_city_adaptor(&params.country, &params.region, &params.name)
        .await
        .map(|v| Json(v))
}

async fn get_cities(
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_adaptor(pagination.page, pagination.page_size())
        .await
        .map(|v| Json(json!(v.payload())))
}

async fn get_city_censuses(
    Path(params): Path<CitiesPathParameters>,
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_censuses_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
    )
    .await
    .map(|v| Json(json!(v.payload())))
}

async fn get_city_ratings(
    Path(params): Path<CitiesPathParameters>,
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_ratings_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
    )
    .await
    .map(|v| Json(json!(v.payload())))
}

async fn post_cities(Json(city): Json<CityPost>) -> Result<Json<Value>, ExecutionError> {
    post_cities_adaptor(city).await.map(|v| Json(v))
}

async fn get_cities_submission(
    Path(submission_id): Path<i32>,
    OptionalQuery(status): OptionalQuery<String>,
) -> Result<Json<Value>, ExecutionError> {
    get_cities_submission_adaptor(submission_id, status)
        .await
        .map(|v| Json(v))
}

async fn get_cities_submissions(
    OptionalQuery(status): OptionalQuery<String>,
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_submissions_adaptor(status, pagination.page, pagination.page_size())
        .await
        .map(|v| Json(json!(v.payload())))
}

async fn patch_cities_submission(
    Query(submission_id): Query<i32>,
    Json(submission): Json<SubmissionPatch>,
) -> Result<Json<Value>, ExecutionError> {
    patch_cities_submission_adaptor(submission_id, submission)
        .await
        .map(|v| Json(v))
}

async fn post_cities_submissions(
    Json(submission): Json<SubmissionPost>,
) -> Result<Json<Value>, ExecutionError> {
    post_cities_submission_adaptor(submission)
        .await
        .map(|v| Json(v))
}
