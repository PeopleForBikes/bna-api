//! Module for the /cities enpoint.

use effortless::{api::parse_path_parameter, error::APIErrors};
use lambda_http::Request;

/// Path parameters for the /cities enpoint.
#[derive(Debug)]
pub struct CitiesPathParameters {
    /// Country name.
    pub country: String,
    /// Region name.
    pub region: String,
    /// City name.
    pub name: String,
}

/// Extract the path parameters for the /cities endpoint.
pub fn extract_path_parameters(event: &Request) -> Result<CitiesPathParameters, APIErrors> {
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
