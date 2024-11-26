use super::adaptor::{
    get_cities_adaptor, get_cities_censuses_adaptor, get_cities_ratings_adaptor,
    get_cities_submission_adaptor, get_cities_submissions_adaptor, get_city_adaptor,
    patch_cities_submission_adaptor, post_cities_adaptor, post_cities_census_adaptor,
    post_cities_submission_adaptor,
};
use crate::{core::resource::cities::CitiesPathParameters, Context, ExecutionError};
use axum::{
    debug_handler,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::OptionalQuery;
use effortless::api::PaginationParameters;
use entity::wrappers::{
    census::CensusFromCityPost,
    city::CityPost,
    submission::{SubmissionPatch, SubmissionPost},
};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new()
        .route("/cities", get(get_cities).post(post_cities))
        .route("/cities/:country/:region/:name", get(get_city))
        .route(
            "/cities/:country/:region/:name/census",
            get(get_city_censuses).post(post_city_census),
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

async fn get_city(
    Path(params): Path<CitiesPathParameters>,
    ctx: Context,
) -> Result<Json<Value>, ExecutionError> {
    get_city_adaptor(&params.country, &params.region, &params.name, ctx)
        .await
        .map(Json)
}

async fn get_cities(pagination: Option<Query<PaginationParameters>>) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_adaptor(pagination.page, pagination.page_size()).await
}

async fn get_city_censuses(
    Path(params): Path<CitiesPathParameters>,
    pagination: Option<Query<PaginationParameters>>,
) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_censuses_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
    )
    .await
}

async fn get_city_ratings(
    Path(params): Path<CitiesPathParameters>,
    pagination: Option<Query<PaginationParameters>>,
    ctx: Context,
) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_ratings_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
        ctx,
    )
    .await
}

async fn post_cities(Json(city): Json<CityPost>) -> Result<Json<Value>, ExecutionError> {
    post_cities_adaptor(city).await.map(Json)
}

async fn get_cities_submission(
    Path(submission_id): Path<i32>,
    OptionalQuery(status): OptionalQuery<String>,
    ctx: Context,
) -> Result<Json<Value>, ExecutionError> {
    // let ctx = Context::new(request_id, source);
    get_cities_submission_adaptor(submission_id, status, ctx)
        .await
        .map(Json)
}

#[derive(Deserialize)]
struct SubmissionParameters {
    pub status: Option<String>,
}

#[debug_handler]
async fn get_cities_submissions(
    Query(submission_params): Query<SubmissionParameters>,
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_submissions_adaptor(
        submission_params.status,
        pagination.page,
        pagination.page_size(),
    )
    .await
    .map(|v| Json(json!(v.payload())))
}

async fn patch_cities_submission(
    Query(submission_id): Query<i32>,
    Json(submission): Json<SubmissionPatch>,
) -> Result<Json<Value>, ExecutionError> {
    patch_cities_submission_adaptor(submission_id, submission)
        .await
        .map(Json)
}

async fn post_cities_submissions(
    Json(submission): Json<SubmissionPost>,
) -> Result<(StatusCode, Json<Value>), ExecutionError> {
    post_cities_submission_adaptor(submission)
        .await
        .map(|v| (StatusCode::CREATED, Json(v)))
}

#[axum::debug_handler]
async fn post_city_census(
    Path(params): Path<CitiesPathParameters>,
    Json(census): Json<CensusFromCityPost>,
) -> Result<(StatusCode, Json<Value>), ExecutionError> {
    post_cities_census_adaptor(&params.country, &params.region, &params.name, census)
        .await
        .map(|v| (StatusCode::CREATED, Json(v)))
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
