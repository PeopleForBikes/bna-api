use super::{
    adaptor::{
        get_rating_adaptor, get_ratings_adaptor, get_ratings_city_adaptor, post_ratings_adaptor,
    },
    schema::{Rating, RatingPost, RatingWithCity, Ratings},
};
use crate::{
    core::resource::schema::{ErrorResponses, PaginationParams},
    Context, ExecutionError, PageFlow, Paginatron,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use effortless::api::PaginationParameters;
use entity::wrappers::bna::BNAPost;
use tracing::debug;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

const TAG: &str = "rating";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_rating))
        .routes(routes!(get_ratings))
        .routes(routes!(post_rating))
        .routes(routes!(get_ratings_city))
}

#[utoipa::path(
  get,
  path = "/ratings/{rating_id}",
  description = "Get the details of a specific city rating",
  tag = TAG,
  params(
    ("rating_id" = Uuid, Path, description = "Rating identifier")
  ),
  responses(
    (status = OK, description = "Fetches the details of a city rating", body = Rating ),
    ErrorResponses,
  ))]
async fn get_rating(
    Path(rating_id): Path<Uuid>,
    ctx: Context,
) -> Result<Json<Rating>, ExecutionError> {
    get_rating_adaptor(rating_id, ctx)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(Rating::from)
        .map(Json)
}

#[utoipa::path(
  get,
  path = "/ratings",
  description = "Get city ratings",
  tag = TAG,
  params(
    PaginationParams,
  ),
  responses(
    (status = OK, description = "Fetches the city ratings", body = Ratings),
  ))]
async fn get_ratings(
    Query(pagination): Query<PaginationParameters>,
) -> Result<PageFlow<Ratings>, ExecutionError> {
    let (total_items, models) =
        get_ratings_adaptor(pagination.page(), pagination.page_size()).await?;

    let payload: Ratings = models.into();
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, pagination.page(), pagination.page_size()),
        payload,
    ))
}

#[utoipa::path(
  post,
  path = "/ratings",
  description = "Create a new city rating",
  tag = TAG,
  request_body = RatingPost,
  responses(
    (status = CREATED, description = "Creates a new city rating", body = Rating ),
    ErrorResponses,
  ))]
async fn post_rating(
    Json(bna): Json<BNAPost>,
) -> Result<(StatusCode, Json<Rating>), ExecutionError> {
    post_ratings_adaptor(bna)
        .await
        .map_err(|e| {
            debug!("{e}");
            e
        })
        .map(Rating::from)
        .map(|v| (StatusCode::CREATED, Json(v)))
}

#[utoipa::path(
  get,
  path = "/ratings/{rating_id}/city",
  description = "Get a city rating and its associated city details",
  tag = TAG,
  params(
    ("rating_id" = Uuid, Path, description = "Rating identifier")
  ),
  responses(
    (status = OK, description = "Fetches a city rating and its associated city details" , body = RatingWithCity ),
    ErrorResponses,
  ))]
async fn get_ratings_city(
    Path(rating_id): Path<Uuid>,
    ctx: Context,
) -> Result<Json<RatingWithCity>, ExecutionError> {
    get_ratings_city_adaptor(rating_id, ctx)
        .await
        .map(|(rating, model)| RatingWithCity {
            rating: rating.into(),
            city: model.into(),
        })
        .map(Json)
}
