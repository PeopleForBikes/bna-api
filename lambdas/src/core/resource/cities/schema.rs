//! Describes the Citi schemas.
use crate::core::resource::schema::{City, Country};
use chrono::DateTime;
use entity::{census, submission, summary};
use serde::{Deserialize, Serialize};
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

#[derive(ToSchema, Serialize)]
#[schema(description = "Census information of a city")]
pub(crate) struct Census {
    /// Census identifier
    id: i32,
    /// City identifier
    city_id: Uuid,
    /// Creation date
    created_at: DateTime<chrono::FixedOffset>,
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

impl From<census::Model> for Census {
    fn from(value: census::Model) -> Self {
        Self {
            id: value.id,
            city_id: value.city_id,
            created_at: value.created_at,
            fips_code: value.fips_code,
            pop_size: value.pop_size,
            population: value.population,
        }
    }
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

// This is the current version which does not seem to be valid OAS3.1
// https://github.com/juhaku/utoipa/issues/901
// https://github.com/juhaku/utoipa/pull/1103
// https://json-schema.org/understanding-json-schema/reference/array#tupleValidation
// struct CityCensuses(Vec<(City, Option<Census>)>);
// So here is the new idea, which shounds more correct anyway
#[derive(ToSchema, Serialize)]
pub(crate) struct CityCensuses {
    pub(crate) city: City,
    pub(crate) censuses: Vec<Census>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct RatingSummary {
    /// Analysis identifier
    id: Uuid,
    /// City identifier
    city_id: Uuid,
    /// Creation date
    created_at: DateTime<chrono::FixedOffset>,
    /// BNA score
    #[schema(examples("77.0"))]
    score: f64,
    /// Analysis version. The format follows the [calver](https://calver.org)
    /// specification with the YY.0M[.Minor] scheme.
    #[schema(examples("23.12"))]
    version: String,
}

impl From<summary::Model> for RatingSummary {
    fn from(value: summary::Model) -> Self {
        Self {
            id: value.id,
            city_id: value.city_id,
            created_at: value.created_at,
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
