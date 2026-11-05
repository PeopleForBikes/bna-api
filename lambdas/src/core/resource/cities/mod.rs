//! Module for the /cities enpoint.
pub mod adaptor;
mod db;
pub mod endpoint;
mod schema;

pub use schema::ApprovalStatus;

use super::schema::Country;
use serde::Deserialize;

/// Path parameters for the /cities enpoint.
#[derive(Debug, Deserialize)]
pub struct CitiesPathParameters {
    /// Country name, one of the supported countries.
    #[serde(deserialize_with = "deserialize_country")]
    pub country: String,
    /// Region name.
    pub region: String,
    /// City name.
    pub name: String,
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
