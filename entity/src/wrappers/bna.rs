use crate::{
    core_services, infrastructure, opportunity, people, recreation, retail, summary, transit,
};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNASummary {
    pub rating_id: Uuid,
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

impl BNARecreation {
    pub fn refresh_score(&mut self) {
        let score = (self.community_centers.unwrap_or_default()
            + self.parks.unwrap_or_default()
            + self.recreation_trails.unwrap_or_default())
            / 3.0;
        self.score = Some(score);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAOpportunity {
    pub employment: Option<f64>,
    pub higher_education: Option<f64>,
    pub k12_education: Option<f64>,
    pub score: Option<f64>,
    pub technical_vocational_college: Option<f64>,
}

impl BNAOpportunity {
    pub fn refresh_score(&mut self) {
        let score = (self.employment.unwrap_or_default()
            + self.higher_education.unwrap_or_default()
            + self.k12_education.unwrap_or_default()
            + self.technical_vocational_college.unwrap_or_default())
            / 4.0;
        self.score = Some(score);
    }
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

impl BNACoreServices {
    pub fn refresh_score(&mut self) {
        let score = (self.dentists.unwrap_or_default()
            + self.doctors.unwrap_or_default()
            + self.grocery.unwrap_or_default()
            + self.hospitals.unwrap_or_default()
            + self.pharmacies.unwrap_or_default()
            + self.social_services.unwrap_or_default())
            / 6.0;
        self.score = Some(score);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAPeople {
    pub score: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNARetail {
    pub score: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNATransit {
    pub score: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAPost {
    pub core_services: BNACoreServices,
    pub infrastructure: BNAInfrastructure,
    pub opportunity: BNAOpportunity,
    pub people: BNAPeople,
    pub recreation: BNARecreation,
    pub retail: BNARetail,
    pub summary: BNASummary,
    pub transit: BNATransit,
}

impl BNAPost {
    pub fn refresh_score(&mut self) {
        let score = (self.core_services.score.unwrap_or_default()
            // + self.features.score.unwrap_or_default()
            + self.opportunity.score.unwrap_or_default()
            + self.recreation.score.unwrap_or_default())
            / 4.0;
        self.summary.score = score;
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNAPatch {
    pub core_services: BNACoreServices,
    pub people: BNAPeople,
    pub retail: BNARetail,
    pub transit: BNATransit,
    pub infrastructure: BNAInfrastructure,
    pub opportunity: BNAOpportunity,
    pub recreation: BNARecreation,
    pub summary: BNASummary,
}

// Note(rgreinho): This struct is name Rating* because the Bna* structs should/will
//  eventually be renamed to Rating*.
#[derive(Debug, Serialize)]
pub struct RatingFlat {
    #[serde(flatten)]
    pub core_services: core_services::Model,
    #[serde(flatten)]
    pub infrastructure: infrastructure::Model,
    #[serde(flatten)]
    pub opportunity: opportunity::Model,
    #[serde(flatten)]
    pub people: people::Model,
    #[serde(flatten)]
    pub recreation: recreation::Model,
    #[serde(flatten)]
    pub retail: retail::Model,
    #[serde(flatten)]
    pub summary: summary::Model,
    #[serde(flatten)]
    pub transit: transit::Model,
}
