use super::{
    db::{fetch_all_reports, fetch_report_year},
    schema::BnaReport,
};
use crate::{database_connect, ExecutionError};

pub(crate) async fn get_report_adaptor(year: u32) -> Result<Vec<BnaReport>, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    Ok(fetch_report_year(&db, year).await?)
}

pub(crate) async fn get_reports_adaptor() -> Result<Vec<BnaReport>, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    Ok(fetch_all_reports(&db).await?)
}
