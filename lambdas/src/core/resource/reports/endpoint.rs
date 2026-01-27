use crate::{database_connect_or_init, ExecutionError};

use super::adaptor::{get_report_adaptor, get_reports_adaptor};
use axum::extract::Path;
use axum_streams::*;
use futures::{stream, StreamExt};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

const TAG: &str = "report";

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_reports))
        .routes(routes!(get_reports_year))
}

#[utoipa::path(
  get,
  path = "/reports",
  description = "Retrieve all rating reports.",
  tag = TAG,
  responses(
    (status = OK, description = "Fetches all rating reports in csv format", content_type = "text/csv"),
  )
)]
async fn get_reports() -> Result<StreamBodyAs<'static>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let ratings = get_reports_adaptor(db).await.expect("reports");
    let stream = stream::iter(ratings);
    Ok(StreamBodyAs::new(
        CsvStreamFormat::new(true, b','),
        stream.map(Ok::<_, axum::Error>),
    ))
}

#[utoipa::path(
  get,
  path = "/reports/{year}",
  description = "Retrieve the latest rating reports for a specific year.",
  tag = TAG,
  params(
    ("year" = u32, Path, description = "Year to retrieve the reports for", minimum = 2017, maximum = 2029)
  ),
  responses(
    (status = OK, description = "Fetches the latest rating reports for a specific year in csv format", content_type = "text/csv"),
  )
)]
async fn get_reports_year(Path(year): Path<u32>) -> Result<StreamBodyAs<'static>, ExecutionError> {
    let db = database_connect_or_init().await?;
    let ratings = get_report_adaptor(db, year).await.expect("reports");
    let stream = stream::iter(ratings);
    Ok(StreamBodyAs::new(
        CsvStreamFormat::new(true, b','),
        stream.map(Ok::<_, axum::Error>),
    ))
}
