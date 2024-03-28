use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNASummary {
    pub bna_uuid: Uuid,
    pub city_id: Uuid,
    pub score: f64,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAInfrastructure {
    pub low_stress_miles: Option<f64>,
    pub high_stress_miles: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNARecreation {
    pub community_centers: Option<f64>,
    pub parks: Option<f64>,
    pub recreation_trails: Option<f64>,
    pub score: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAOpportunity {
    pub employment: Option<f64>,
    pub higher_education: Option<f64>,
    pub k12_education: Option<f64>,
    pub score: Option<f64>,
    pub technical_vocational_college: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNACoreServices {
    pub dentists: Option<f64>,
    pub doctors: Option<f64>,
    pub grocery: Option<f64>,
    pub hospitals: Option<f64>,
    pub pharmacies: Option<f64>,
    pub score: Option<f64>,
    pub social_services: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAFeatures {
    pub people: Option<f64>,
    pub retail: Option<f64>,
    pub transit: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAPost {
    pub core_services: BNACoreServices,
    pub features: BNAFeatures,
    pub infrastructure: BNAInfrastructure,
    pub opportunity: BNAOpportunity,
    pub recreation: BNARecreation,
    pub summary: BNASummary,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAPatch {
    pub core_services: BNACoreServices,
    pub features: BNAFeatures,
    pub infrastructure: BNAInfrastructure,
    pub opportunity: BNAOpportunity,
    pub recreation: BNARecreation,
    pub summary: BNASummary,
}
