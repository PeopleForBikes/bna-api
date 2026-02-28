use utoipa_axum::{router::OpenApiRouter, routes};

const TAG: &str = "system";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(get_health))
}

#[utoipa::path(
  get,
  path = "/health",
  description = "Get the healthcheck of the system.",
  tag = TAG,
  responses(
    (status = OK, description = "Fetches system healthcheck", body = String),
  ))]
async fn get_health() -> &'static str {
    "OK"
}
