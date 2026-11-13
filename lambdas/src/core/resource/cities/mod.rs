//! Module for the /cities endpoint.
pub mod adaptor;
mod db;
pub mod endpoint;
mod schema;

pub use schema::ApprovalStatus;

use super::schema::{deserialize_path_name, Country};
use serde::Deserialize;

/// Path parameters for the /cities endpoint.
#[derive(Debug, Deserialize)]
pub struct CitiesPathParameters {
    /// Country name, one of the supported countries.
    #[serde(deserialize_with = "deserialize_country")]
    pub country: String,
    /// Region name.
    #[serde(deserialize_with = "deserialize_region")]
    pub region: String,
    /// City name.
    #[serde(deserialize_with = "deserialize_name")]
    pub name: String,
}

/// Deserializes a region name, of a bounded length.
fn deserialize_region<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_path_name("region", deserializer)
}

/// Deserializes a city name, of a bounded length.
fn deserialize_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_path_name("name", deserializer)
}

/// Deserializes a country name, rejecting the ones that are not supported.
fn deserialize_country<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let country = String::deserialize(deserializer)?;
    match country.parse::<Country>() {
        Ok(_) => Ok(country),
        Err(_) => Err(serde::de::Error::custom(format!(
            "unsupported country: {country}"
        ))),
    }
}
