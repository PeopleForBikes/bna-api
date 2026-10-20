//! Describes the schemas shared accross resources.
use crate::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use chrono::DateTime;
use entity::city;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::convert::Into;
use std::{fmt::Display, str::FromStr};
use utoipa::{IntoParams, IntoResponses, ToSchema};
use uuid::Uuid;

#[derive(ToSchema, Serialize, Deserialize)]
#[schema(description = "Supported countries")]
pub(crate) enum Country {
    Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    #[serde(rename = "Antigua and Barbuda")]
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Australia,
    Austria,
    Azerbaijan,
    #[serde(rename = "The Bahamas")]
    TheBahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bhutan,
    Bolivia,
    #[serde(rename = "Bosnia and Herzegovina")]
    BosniaAndHerzegovina,
    Botswana,
    Brazil,
    Brunei,
    Bulgaria,
    #[serde(rename = "Burkina Faso")]
    BurkinaFaso,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    #[serde(rename = "Cape Verde")]
    CapeVerde,
    #[serde(rename = "Central African Republic")]
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    Congo,
    #[serde(rename = "Costa Rica")]
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    Czechia,
    #[serde(rename = "Côte d'Ivoire")]
    CoteDIvoire,
    #[serde(rename = "Democratic Republic of the Congo")]
    DemocraticRepublicOfTheCongo,
    Denmark,
    Djibouti,
    Dominica,
    #[serde(rename = "Dominican Republic")]
    DominicanRepublic,
    #[serde(rename = "East Timor")]
    EastTimor,
    Ecuador,
    Egypt,
    #[serde(rename = "El Salvador")]
    ElSalvador,
    England,
    #[serde(rename = "Equatorial Guinea")]
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Eswatini,
    Ethiopia,
    Fiji,
    Finland,
    France,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    #[serde(rename = "Guinea-Bissau")]
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Kosovo,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    #[serde(rename = "Marshall Islands")]
    MarshallIslands,
    Mauritania,
    Mauritius,
    Mexico,
    Micronesia,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    #[serde(rename = "New Zealand")]
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    #[serde(rename = "North Korea")]
    NorthKorea,
    #[serde(rename = "North Macedonia")]
    NorthMacedonia,
    #[serde(rename = "Northern Ireland")]
    NorthernIreland,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Palestine,
    Panama,
    #[serde(rename = "Papua New Guinea")]
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Qatar,
    #[serde(rename = "Republic of the Congo")]
    RepublicOfTheCongo,
    Romania,
    Russia,
    Rwanda,
    #[serde(rename = "Saint Kitts and Nevis")]
    SaintKittsAndNevis,
    #[serde(rename = "Saint Lucia")]
    SaintLucia,
    #[serde(rename = "Saint Vincent and the Grenadines")]
    SaintVincentAndTheGrenadines,
    Samoa,
    #[serde(rename = "San Marino")]
    SanMarino,
    #[serde(rename = "São Tomé and Príncipe")]
    SaoTomeAndPrincipe,
    #[serde(rename = "Saudi Arabia")]
    SaudiArabia,
    Scotland,
    Senegal,
    Serbia,
    Seychelles,
    #[serde(rename = "Sierra Leone")]
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    #[serde(rename = "Solomon Islands")]
    SolomonIslands,
    Somalia,
    #[serde(rename = "South Africa")]
    SouthAfrica,
    #[serde(rename = "South Korea")]
    SouthKorea,
    #[serde(rename = "South Sudan")]
    SouthSudan,
    Spain,
    #[serde(rename = "Sri Lanka")]
    SriLanka,
    Sudan,
    Suriname,
    Sweden,
    Switzerland,
    Syria,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    #[serde(rename = "Timor-Leste")]
    TimorLeste,
    Togo,
    Tonga,
    #[serde(rename = "Trinidad and Tobago")]
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Uganda,
    Ukraine,
    #[serde(rename = "United Arab Emirates")]
    UnitedArabEmirates,
    #[serde(rename = "United Kingdom")]
    UnitedKingdom,
    #[serde(rename = "United States")]
    UnitedStates,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    #[serde(rename = "Vatican City")]
    VaticanCity,
    Venezuela,
    Vietnam,
    Wales,
    #[serde(rename = "West Bank")]
    WestBank,
    Yemen,
    Zambia,
    Zimbabwe,
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
        write!(f, "{value}")
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
    fn from(value: city::Model) -> Self {
        Self {
            id: value.id,
            country: value.country.into(),
            state: value.state,
            name: value.name,
            latitude: value.latitude,
            longitude: value.longitude,
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

/// Response for an operation reading or updating a resource that may not exist.
#[allow(dead_code)]
#[derive(IntoResponses)]
pub(crate) enum NotFoundResponse {
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

/// Highest page number a client can request.
const MAX_PAGE: u64 = 65536;

/// First year with BNA results.
pub(crate) const FIRST_BNA_YEAR: i32 = 2017;

/// Last year that can be requested.
pub(crate) const LAST_BNA_YEAR: i32 = 2029;

/// Deserializes a BNA year, between `FIRST_BNA_YEAR` and `LAST_BNA_YEAR`.
pub(crate) fn deserialize_bna_year<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let year = i32::deserialize(deserializer)?;
    if (FIRST_BNA_YEAR..=LAST_BNA_YEAR).contains(&year) {
        Ok(year)
    } else {
        Err(serde::de::Error::custom(format!(
            "the year must be between {FIRST_BNA_YEAR} and {LAST_BNA_YEAR}"
        )))
    }
}

/// Response for an operation creating a resource that may already exist.
#[allow(dead_code)]
#[derive(IntoResponses)]
pub(crate) enum ConflictResponse {
    /// Conflict
    #[response(
        status = 409,
        description = "The resource already exists.",
        example = json!({
          "errors": [
          {
            "details": "the resource already exists",
            "id": "blfwkg8nvHcEJnQ=",
            "source": {"pointer": "/cities"},
            "status": "409",
            "title": "Conflict"
          }
        ]})
    )]
    Conflict(APIErrors),
}

/// Response for an operation receiving parameters or a body that may be invalid.
#[allow(dead_code)]
#[derive(IntoResponses)]
pub(crate) enum BadRequestResponse {
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
}

/// Deserializes a page number, between 1 and `MAX_PAGE`.
pub(crate) fn deserialize_page<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let page = Option::<u64>::deserialize(deserializer)?;
    match page {
        Some(0) => Err(serde::de::Error::custom("pages start at 1")),
        Some(p) if p > MAX_PAGE => Err(serde::de::Error::custom(format!(
            "pages go up to {MAX_PAGE}"
        ))),
        _ => Ok(page),
    }
}

/// Deserializes a page size, between 1 and `MAX_PAGE_SIZE`.
pub(crate) fn deserialize_page_size<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let page_size = Option::<u64>::deserialize(deserializer)?;
    match page_size {
        Some(s) if !(1..=MAX_PAGE_SIZE).contains(&s) => Err(serde::de::Error::custom(format!(
            "the page size must be between 1 and {MAX_PAGE_SIZE}"
        ))),
        _ => Ok(page_size),
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub(crate) struct PaginationParameters {
    /// The number of items per page
    #[param(minimum = 1, maximum = 100, example = 25)]
    #[serde(default, deserialize_with = "deserialize_page_size")]
    page_size: Option<u64>,
    /// The result page being returned, starting at 1. A page past the last one returns an empty list.
    #[param(minimum = 1, maximum = 65536, example = 5)]
    #[serde(default, deserialize_with = "deserialize_page")]
    page: Option<u64>,
}

impl Default for PaginationParameters {
    fn default() -> Self {
        Self {
            page_size: Some(DEFAULT_PAGE_SIZE),
            page: Some(1),
        }
    }
}

impl PaginationParameters {
    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(DEFAULT_PAGE_SIZE)
    }

    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1)
    }
}

#[derive(Debug, Default, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(description = "Order direction for sorting")]
pub enum OrderDirection {
    #[default]
    Asc,
    Desc,
}

impl From<sea_orm::Order> for OrderDirection {
    fn from(value: sea_orm::Order) -> Self {
        match value {
            sea_orm::Order::Desc => OrderDirection::Desc,
            _ => OrderDirection::Asc,
        }
    }
}

impl From<OrderDirection> for sea_orm::Order {
    fn from(value: OrderDirection) -> Self {
        match value {
            OrderDirection::Desc => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        }
    }
}

/// Sorting parameters shared across all list endpoints.
#[allow(dead_code)]
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub(crate) struct SortParameters {
    /// Field to sort by.
    #[param(example = "created_at")]
    sort_by: Option<String>,

    /// Sort direction (`asc` or `desc`).
    #[param(example = "desc")]
    order_direction: Option<OrderDirection>,

    /// Fetch the latest entry.
    /// When specified, it ignores the `sort_by` and `order_direction` parameters.
    /// This parameter does not apply to all entities.
    #[param(example = "true")]
    latest: Option<bool>,
}

impl Default for SortParameters {
    fn default() -> Self {
        Self {
            sort_by: Some(String::from("created_at")),
            order_direction: Some(OrderDirection::default()),
            latest: Some(false),
        }
    }
}

/// List parameters combining sorting and pagination.
#[allow(dead_code)]
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[schema(as = ListParameters)]
pub(crate) struct ListParams {
    #[serde(flatten)]
    pagination: Option<PaginationParameters>,

    #[serde(flatten)]
    sorting: Option<SortParameters>,
}
