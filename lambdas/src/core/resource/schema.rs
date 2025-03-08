//! Describes the schemas shared accross resources.
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fmt::Display, str::FromStr};
use utoipa::{IntoParams, IntoResponses, ToSchema};
use uuid::Uuid;

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) enum Country {
    Australia,
    Belgium,
    Brazil,
    Canada,
    Chile,
    Colombia,
    Croatia,
    Cuba,
    England,
    France,
    Germany,
    Greece,
    Guatemala,
    Iran,
    Iraq,
    Ireland,
    Italy,
    Mexico,
    Netherlands,
    #[serde(rename = "New Zealand")]
    NewZealand,
    #[serde(rename = "Northern Ireland")]
    NorthernIreland,
    Portugal,
    Scotland,
    Spain,
    #[serde(rename = "United States")]
    UnitedStates,
    Vietnam,
    Wales,
}

impl FromStr for Country {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl From<String> for Country {
    fn from(value: String) -> Self {
        Country::from_str(value.as_str()).expect("cannot serialize value")
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{}", value)
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
#[schema(description = "Detailed information of a city")]
pub(crate) struct City {
    /// City identifier
    id: Uuid,
    /// Country name
    #[schema(examples("Belgium"))]
    country: Country,
    /// State name
    #[schema(examples("Antwerp"))]
    state: String,
    /// City name
    #[schema(examples("Antwerp"))]
    name: String,
    /// Geographic coordinate that specifies the north-south position of a point
    /// on the surface of the Earth.
    #[schema(examples("51.260197"))]
    latitude: Option<f64>,
    /// Geographic coordinate that specifies the east–west position of a point
    /// on the surface of the Earth.
    #[schema(examples("4.402771"))]
    longitude: Option<f64>,
    /// Region name. A region can be a state, a province, a community, or
    /// something similar depending on the country. If a country does not have
    /// this concept, then the country name is used.
    #[schema(examples("Antwerp"))]
    region: Option<String>,
    /// A short version of the state name, usually 2 or 3 character long
    #[schema(examples("VAN"))]
    state_abbrev: Option<String>,
    /// Residential speed limit in kilometer per hour (km/h).
    /// Only use if different from the state speed limit.
    #[schema(examples("50"))]
    residential_speed_limit: Option<i32>,
    /// Creation date
    created_at: DateTime<chrono::FixedOffset>,
    /// Update date
    updated_at: Option<DateTime<chrono::FixedOffset>>,
}

impl From<entity::city::Model> for City {
    fn from(value: entity::city::Model) -> Self {
        Self {
            id: value.id,
            country: value.country.into(),
            state: value.state,
            name: value.name,
            latitude: value.latitude,
            longitude: value.latitude,
            region: value.region,
            state_abbrev: value.state_abbrev,
            residential_speed_limit: value.residential_speed_limit,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

/// An object containing references to the primary source of the error.
///
/// It SHOULD include one of the following members or be omitted:
///
///   - pointer: a JSON Pointer [RFC6901](https://tools.ietf.org/html/rfc6901) to the
///     value in the request document that caused the error [e.g. "/data" for a primary
///     data object, or "/data/attributes/title" for a specific attribute].
///     This MUST point to a value in the request document that exists; if it doesn’t,
///     the client SHOULD simply ignore the pointer.
///   - parameter: a string indicating which URI query parameter caused the error.
///   - header: a string indicating the name of a single request header which caused the
///     error.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorSource {
    /// A JSON Pointer [RFC6901] to the value in the request document that caused the error.
    #[schema(examples(json!({"pointer": "/data/attributes/title"})))]
    Pointer(String),
    /// A string indicating which URI query parameter caused the error.
    #[schema(examples(json!({ "parameter": "include" })))]
    Parameter(String),
    /// A string indicating the name of a single request header which caused the error.
    #[schema(examples(json!({ "header": "Content-Type" })))]
    Header(String),
}

/// Single API Error object as described in <https://jsonapi.org/format/#error-objects>.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct APIError {
    /// A unique identifier for this particular occurrence of the problem.
    #[schema(examples("blfwkg8nvHcEJnQ="))]
    id: Option<String>,
    /// The HTTP status code applicable to this problem, expressed as a string value.
    #[schema(examples("404"))]
    status: String,
    /// A short, human-readable summary of the problem
    #[schema(examples("Item Not Found"))]
    title: String,
    /// A human-readable explanation specific to this occurrence of the problem
    #[schema(examples("the entry was not found"))]
    details: String,
    /// An object containing references to the primary source of the error.
    ///
    /// This field may be omitted in some situation. For instance, if the server cannot
    /// parse the request as valid JSON, including source doesn’t make sense
    /// (because there’s no JSON document for source to refer to).
    #[schema(examples(json!("pointer": "/bnas/analysis/e6aade5a-b343-120b-dbaa-bd916cd99221")))]
    source: APIErrorSource,
}

/// Error objects MUST be returned as an array keyed by errors in the top level of a
/// JSON:API document.
#[derive(Deserialize, Serialize, ToSchema)]
pub struct APIErrors {
    pub errors: Vec<APIError>,
}

#[allow(dead_code)]
#[derive(Serialize, IntoResponses, ToSchema)]
pub(crate) enum ErrorResponses {
    /// Bad Request
    #[response(
        status = 400,
        description = "The request was formatted incorrectly or missing required parameters.",
        example = json!({
          "errors": [
          {
            "details": "the request was formatted incorrectly or missing required parameters",
            "id": "blfwkg8nvHcEJnQ=",
            "source": {"parameter": "status"},
            "status": "400",
            "title": "Bad Request"
          }
        ]})
    )]
    BadRequest(APIErrors),
    /// Unauthorized
    #[response(
      status = 401,
      description = "The request has not been fulfilled because it lacks valid authentication credentials for the target resource.",
      example = json!({
        "errors": [
        {
          "details": "invalid authentication credentials to access the specified resource",
          "id": "blfwkg8nvHcEJnQ=",
          "source": {"pointer": "/bnas/ratings/e6aade5a-b343-120b-dbaa-bd916cd99221"},
          "status": "401",
          "title": "Unauthorized"
        }
      ]})
  )]
    Unauthorized(APIErrors),
    /// Forbidden
    #[response(
      status = 403,
      description = "Forbidden to make the request. Most likely this indicates an issue with the credentials or permissions.",
      example = json!({
        "errors": [
        {
          "details": "access to the requested resource is forbidden",
          "id": "blfwkg8nvHcEJnQ=",
          "source": {"pointer": "/bnas/ratings/e6aade5a-b343-120b-dbaa-bd916cd99221"},
          "status": "403",
          "title": "Forbidden"
        }
      ]})
  )]
    Forbidden(APIErrors),
    /// Item Not Found
    #[response(
        status = 404,
        description = "The particular resource requested was not found. This occurs, for example, when the id of the requested  resource does not exist.",
        example = json!({
          "errors": [
            {
              "details": "the resource was not found",
              "id": "blfwkg8nvHcEJnQ=",
              "source": {"pointer": "/bnas/ratings/e6aade5a-b343-120b-dbaa-bd916cd99221"},
              "status": "404",
              "title": "Item Not Found"
            }
          ]})
    )]
    NotFound(APIErrors),
}

#[allow(dead_code)]
#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub(crate) struct PaginationParams {
    /// The number of items per page
    #[param(minimum = 1, maximum = 65536, example = 25)]
    page_size: Option<u64>,
    /// The result page being returned
    #[param(minimum = 1, maximum = 65536, example = 5)]
    page: Option<u64>,
}
