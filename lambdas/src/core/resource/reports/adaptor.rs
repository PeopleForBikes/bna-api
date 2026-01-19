use sea_orm::DatabaseConnection;

use super::{
    db::{fetch_all_reports, fetch_report_year},
    schema::BnaReport,
};
use crate::ExecutionError;

pub(crate) async fn get_report_adaptor(
    db: &DatabaseConnection,
    year: u32,
) -> Result<Vec<BnaReport>, ExecutionError> {
    Ok(fetch_report_year(db, year).await?)
}

pub(crate) async fn get_reports_adaptor(
    db: &DatabaseConnection,
) -> Result<Vec<BnaReport>, ExecutionError> {
    Ok(fetch_all_reports(db).await?)
}
