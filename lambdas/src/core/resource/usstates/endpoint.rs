use crate::{
    core::resource::{
        schema::{ErrorResponses, PaginationParameters},
        usstates::{
            adaptor::{get_us_state_adaptor, get_us_states_adaptor},
            schema::{UsState, UsStates},
        },
    },
    database_connect_or_init, Context, ExecutionError, PageFlow, Paginatron,
};
use axum::{
    extract::{Path, Query},
    Json,
};
use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "usstate";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_us_state))
        .routes(routes!(get_us_states))
}

#[utoipa::path(
  get,
  path = "/usstates/{name}",
  description = "Get the details of a specific US state.",
  tag = TAG,
  params(
    ("name" = String, Path, description = "Full name of a US state", example = "Texas"),
  ),
  responses(
    (status = OK, description = "Fetches a Us state", body = UsState),
    ErrorResponses,
  ))]
async fn get_us_state(
    Path(name): Path<String>,
    ctx: Context,
) -> Result<Json<UsState>, ExecutionError> {
    let db = database_connect_or_init().await?;
    get_us_state_adaptor(db, &name, ctx)
        .await
        .map(UsState::from)
        .map(Json)
}

#[axum::debug_handler]
#[utoipa::path(
  get,
  path = "/usstates",
  description = "Get the details of all US states.",
  tag = TAG,
  params(
    PaginationParameters,
  ),
  responses(
    (status = OK, description = "Fetches US states", body = UsStates),
  ))]
async fn get_us_states(
    Query(pagination): Query<PaginationParameters>,
) -> Result<PageFlow<UsStates>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let payload = get_us_states_adaptor(db, pagination.page(), pagination.page_size()).await?;
    Ok(PageFlow::new(
        Paginatron::new(None, payload.0, pagination.page(), pagination.page_size()),
        payload.1.into(),
    ))
}
