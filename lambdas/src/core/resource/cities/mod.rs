//! Module for the /cities enpoint.
pub mod adaptor;
mod db;
pub mod endpoint;
mod schema;

pub use schema::ApprovalStatus;

use super::schema::Country;
use effortless::{api::parse_path_parameter, error::APIErrors};
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

/// Extract the path parameters for the /cities endpoint.
pub fn extract_path_parameters(
    event: &lambda_http::Request,
) -> Result<CitiesPathParameters, APIErrors> {
    let mut api_errors = APIErrors::empty();

    let country = match parse_path_parameter::<String>(event, "country") {
        Ok(value) => value,
        Err(e) => {
            api_errors.extend(e);
            None
        }
    };

    let region = match parse_path_parameter::<String>(event, "region") {
        Ok(value) => value,
        Err(e) => {
            api_errors.extend(e);
            None
        }
    };
    let name = match parse_path_parameter::<String>(event, "name") {
        Ok(value) => value,
        Err(e) => {
            api_errors.extend(e);
            None
        }
    };

    if !api_errors.is_empty() {
        return Err(api_errors);
    }

    Ok(CitiesPathParameters {
        country: country.unwrap(),
        region: region.unwrap(),
        name: name.unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{http, Body, RequestExt};
    use std::collections::HashMap;

    #[test]
    fn test_extract_path_parameters() {
        let country: String = String::from("United States");
        let region: String = String::from("Texas");
        let name: String = String::from("Austin");
        let event = http::Request::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::Empty)
            .expect("failed to build request")
            .with_path_parameters(HashMap::from([
                ("country".to_string(), country.clone()),
                ("region".to_string(), region.clone()),
                ("name".to_string(), name.clone()),
            ]))
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let r = extract_path_parameters(&event).unwrap();
        assert_eq!(r.country, country);
        assert_eq!(r.region, region);
        assert_eq!(r.name, name);
    }
}
