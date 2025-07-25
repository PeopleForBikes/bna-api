//! Describes the Citi schemas.
use crate::core::resource::schema::{City, Country};
use chrono::DateTime;
use entity::{submission, summary};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(ToSchema, Serialize)]
pub(crate) struct Cities(Vec<City>);

impl From<Vec<entity::city::Model>> for Cities {
    fn from(value: Vec<entity::city::Model>) -> Self {
        let c = value.into_iter().map(City::from).collect::<Vec<City>>();
        Cities(c)
    }
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct CityPost {
    /// Country name
    #[schema(examples("Belgium"))]
    country: Country,
    /// Geographic coordinate that specifies the north-south position of a point
    /// on the surface of the Earth.
    #[schema(examples("51.260197"))]
    latitude: Option<f64>,
    /// Geographic coordinate that specifies the east–west position of a point
    /// on the surface of the Earth.
    #[schema(examples("4.402771"))]
    longitude: Option<f64>,
    /// City name
    #[schema(examples("Antwerp"))]
    name: String,
    /// Region name. A region can be a state, a province, a community, or
    /// something similar depending on the country. If a country does not have
    /// this concept, then the country name is used.
    #[schema(examples("Antwerp"))]
    region: Option<String>,
    /// A short version of the state name, usually 2 or 3 character long
    #[schema(examples("VAN"))]
    state: String,
    /// A short version of the state name, usually 2 or 3 character long
    #[schema(examples("VAN"))]
    state_abbrev: Option<String>,
    /// Speed limit in kilometer per hour (km/h).
    #[schema(examples("50"))]
    speed_limit: Option<i32>,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct CensusPost {
    /// Numerical city identifier given by the U.S. census, or 0 for non-US cities
    #[schema(examples("4805000"))]
    fips_code: String,
    /// City population size category (small (0), medium (1), large (2))
    #[schema(examples("1"))]
    pop_size: i32,
    /// City population
    #[schema(examples("907779"))]
    population: i32,
}

#[derive(ToSchema, Serialize, FromQueryResult)]
pub(crate) struct RatingSummary {
    /// Analysis identifier
    id: Uuid,
    /// City identifier
    city_id: Uuid,
    /// Creation date
    created_at: DateTime<chrono::FixedOffset>,
    /// City population size category (small, medium, large).
    #[schema(examples("large"))]
    pub pop_size: i32,
    /// City population based on the annual U.S. Census American Community Survey.
    #[schema(examples("989252"))]
    pub population: i32,
    /// Residential speed limit override.
    pub residential_speed_limit_override: Option<i32>,
    /// BNA score
    #[schema(examples("77.0"))]
    score: f64,
    /// Analysis version. The format follows the [calver](https://calver.org)
    /// specification with the YY.0M[.Minor] scheme.
    #[schema(examples("23.12"))]
    version: String,
}

/*
*/

impl From<summary::Model> for RatingSummary {
    fn from(value: summary::Model) -> Self {
        Self {
            city_id: value.city_id,
            created_at: value.created_at,
            id: value.id,
            pop_size: value.pop_size,
            population: value.population,
            residential_speed_limit_override: value.residential_speed_limit_override,
            score: value.score,
            version: value.version,
        }
    }
}

#[derive(ToSchema, Serialize)]
pub(crate) struct CityRatings {
    pub(crate) city: City,
    pub(crate) ratings: Vec<RatingSummary>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Submission {
    /// Submission identifier
    id: i32,
    /// First name
    #[schema(examples("Jane"))]
    first_name: String,
    /// Last name
    last_name: String,
    #[schema(examples("Doe"))]
    /// Job title or position
    #[schema(examples("CTO"))]
    occupation: Option<String>,
    /// Organization or company
    #[schema(examples("Organization LLC"))]
    organization: Option<String>,
    /// email address
    #[schema(examples("jane.doe@orgllc.com"))]
    email: String,
    /// Country
    #[schema(examples("Belgium"))]
    country: Country,
    /// City name
    #[schema(examples("Antwerp"))]
    city: String,
    /// Region name. A region can be a state, a province, a community, or
    /// something similar depending on the country. If a country does not have
    /// this concept, then the country name is used.
    #[schema(examples("Antwerp"))]
    region: Option<String>,
    /// Numerical city identifier given by the U.S. census, or 0 for non-US cities
    #[schema(examples("4805000"))]
    fips_code: String,
    /// Consent status
    #[schema(examples("true"))]
    consent: bool,
    /// Submission status, e.g. "Pending"
    #[schema(examples("Pending"))]
    status: String,
    /// Creation date
    created_at: DateTime<chrono::FixedOffset>,
}

impl From<submission::Model> for Submission {
    fn from(value: submission::Model) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            occupation: value.occupation,
            organization: value.organization,
            email: value.email,
            country: value.country.into(),
            city: value.city,
            region: value.region,
            fips_code: value.fips_code,
            consent: value.consent,
            status: value.status,
            created_at: value.created_at,
        }
    }
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct SubmissionPost {
    /// First name
    #[schema(examples("Jane"))]
    first_name: String,
    /// Last name
    last_name: String,
    #[schema(examples("Doe"))]
    /// Job title or position
    #[schema(examples("CTO"))]
    occupation: Option<String>,
    /// Organization or company
    #[schema(examples("Organization LLC"))]
    organization: Option<String>,
    /// email address
    #[schema(examples("jane.doe@orgllc.com"))]
    email: String,
    /// Country
    #[schema(examples("Belgium"))]
    country: Country,
    /// City name
    #[schema(examples("Antwerp"))]
    city: String,
    /// Region name. A region can be a state, a province, a community, or
    /// something similar depending on the country. If a country does not have
    /// this concept, then the country name is used.
    #[schema(examples("Antwerp"))]
    region: Option<String>,
    /// Numerical city identifier given by the U.S. census, or 0 for non-US cities
    #[schema(examples("4805000"))]
    fips_code: String,
    /// Consent status
    #[schema(examples("true"))]
    consent: bool,
    /// Submission status, e.g. "Pending"
    #[schema(examples("Pending"))]
    status: String,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct SubmissionPatch {
    /// First name
    #[schema(examples("Jane"))]
    first_name: String,
    /// Last name
    last_name: String,
    #[schema(examples("Doe"))]
    /// Job title or position
    #[schema(examples("CTO"))]
    occupation: Option<String>,
    /// Organization or company
    #[schema(examples("Organization LLC"))]
    organization: Option<String>,
    /// email address
    #[schema(examples("jane.doe@orgllc.com"))]
    email: String,
    /// Country
    #[schema(examples("Belgium"))]
    country: Country,
    /// City name
    #[schema(examples("Antwerp"))]
    city: String,
    /// Region name. A region can be a state, a province, a community, or
    /// something similar depending on the country. If a country does not have
    /// this concept, then the country name is used.
    #[schema(examples("Antwerp"))]
    region: Option<String>,
    /// Numerical city identifier given by the U.S. census, or 0 for non-US cities
    #[schema(examples("4805000"))]
    fips_code: String,
    /// Consent status
    #[schema(examples("true"))]
    consent: bool,
    /// Submission status, e.g. "Pending"
    #[schema(examples("Pending"))]
    status: String,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Submissions(pub(crate) Vec<Submission>);

#[allow(dead_code)]
#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub(crate) struct CityParams {
    /// Country name
    #[param(example = "Belgium")]
    country: Country,
    /// Region name. A region can be a state, a province, a community, or
    /// something similar depending on the country. If a country does not have
    /// this concept, then the country name is used.
    #[param(example = "Antwerp")]
    region: String,
    /// City name
    #[param(example = "Antwerp")]
    name: String,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct CityWithSummary {
    pub(crate) city: City,
    pub(crate) sumary: RatingSummary,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct CitiesWithSummary(pub(crate) Vec<CityWithSummary>);
