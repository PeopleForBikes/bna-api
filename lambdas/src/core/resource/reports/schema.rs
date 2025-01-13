use sea_orm::FromQueryResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromQueryResult, Serialize, ToSchema)]
#[schema(description = "A city rating report.")]
pub struct BnaReport {
    // BNA Summary
    pub(crate) id: Uuid,
    pub(crate) score: f64,
    pub(crate) version: String,

    // BNAInfrastructure
    pub(crate) low_stress_miles: Option<f64>,
    pub(crate) high_stress_miles: Option<f64>,

    // BNA Recreation
    pub(crate) community_centers: Option<f64>,
    pub(crate) parks: Option<f64>,
    pub(crate) recreation_trails: Option<f64>,
    pub(crate) recreation_score: Option<f64>,

    // BNA Opportunity
    pub(crate) employment: Option<f64>,
    pub(crate) higher_education: Option<f64>,
    pub(crate) k12_education: Option<f64>,
    pub(crate) opportunity_score: Option<f64>,
    pub(crate) technical_vocational_college: Option<f64>,

    // BNA Core Services
    pub(crate) dentists: Option<f64>,
    pub(crate) doctors: Option<f64>,
    pub(crate) grocery: Option<f64>,
    pub(crate) hospitals: Option<f64>,
    pub(crate) pharmacies: Option<f64>,
    pub(crate) coreservices_score: Option<f64>,
    pub(crate) social_services: Option<f64>,

    // BNA People
    pub(crate) people: Option<f64>,

    // BNA Retail
    pub(crate) retail: Option<f64>,

    // BNA Transit
    pub(crate) transit: Option<f64>,
}

#[allow(dead_code)]
#[derive(ToSchema, Serialize)]
#[schema(description = "A collection of city rating reports.")]
pub(crate) struct BnaReports(Vec<BnaReport>);
