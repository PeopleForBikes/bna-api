//! Module for the /cities enpoint.
use axum::response::{IntoResponse, Response};
use effortless::{
    api::parse_path_parameter,
    error::{APIError, APIErrors},
};
use lambda_http::{
    http::{header, StatusCode},
    Request,
};
use serde::Deserialize;
use serde_json::json;
use thiserror;

pub mod adaptor;
mod db;
pub mod endpoint;

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    ///  An error from unsuccessful database operations.
    #[error("database error")]
    DatabaseError(#[from] sea_orm::DbErr),

    /// Unexpected error.
    #[error("unexpected error: {0} {1}")]
    Unexpected(String, String),

    /// Entry not found.
    #[error("entry not found: {0} {1}")]
    NotFound(String, String),

    /// Uncovered country.
    #[error("the country is not covered by the analyzer: {0}")]
    UncoveredCountry(String),

    /// Invalid US State.
    #[error("the state is not a valid US state: {0}")]
    InvalidUSState(String),
}

impl From<ExecutionError> for APIError {
    fn from(value: ExecutionError) -> Self {
        match value {
            ExecutionError::DatabaseError(_) => APIError::db_error(None, "Unknown source", ""),
            ExecutionError::NotFound(source, message) => {
                APIError::not_found(None, &source, &message)
            }
            _ => APIError::internal_error(
                None,
                "Internal Error",
                "Unknown details",
                "Unknown source",
            ),
        }
    }
}

impl From<ExecutionError> for APIErrors {
    fn from(value: ExecutionError) -> Self {
        APIError::from(value).into()
    }
}

impl IntoResponse for ExecutionError {
    fn into_response(self) -> Response {
        let api_error = APIError::from(self);
        let api_errors = APIErrors::from(api_error);
        let status = if api_errors.errors.len() == 1 {
            api_errors.errors().first().unwrap().status()
        } else {
            StatusCode::BAD_REQUEST
        };
        Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!(api_errors).to_string().into())
            .unwrap()
    }
}

/// Path parameters for the /cities enpoint.
#[derive(Debug, Deserialize)]
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
