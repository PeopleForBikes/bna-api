use crate::core::resource::schema::deserialize_path_name;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize)]
#[schema(description = "Detailed information of a city")]
pub(crate) struct UsState {
    /// State name.
    #[schema(examples("Texas"))]
    name: String,
    /// Two-letter state abbreviation..
    #[schema(examples("TX"))]
    abbrev: String,
    /// State FIPS code.
    #[schema(examples("48"))]
    fipscode: String,
    /// State speed limit in mph.
    #[schema(examples("30"))]
    speed_limit: i32,
}

impl From<entity::us_state::Model> for UsState {
    fn from(value: entity::us_state::Model) -> Self {
        Self {
            name: value.name,
            abbrev: value.abbrev,
            fipscode: value.fips_code,
            speed_limit: value.speed_limit,
        }
    }
}

#[derive(ToSchema, Serialize)]
pub(crate) struct UsStates(Vec<UsState>);

impl From<Vec<entity::us_state::Model>> for UsStates {
    fn from(value: Vec<entity::us_state::Model>) -> Self {
        let s = value
            .into_iter()
            .map(UsState::from)
            .collect::<Vec<UsState>>();
        UsStates(s)
    }
}

/// Path parameters of `GET /usstates/{name}`.
#[derive(Debug, Deserialize)]
pub(crate) struct UsStatePathParameters {
    /// Full name of the US state.
    #[serde(deserialize_with = "deserialize_name")]
    pub(crate) name: String,
}

/// Deserializes a US state name, of a bounded length.
fn deserialize_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_path_name("name", deserializer)
}
