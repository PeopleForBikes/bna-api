use crate::{
    error::{APIError, APIErrorSource, APIErrors},
    fragment::{self, get_apigw_request_id},
};
use lambda_http::{http::StatusCode, Request};
use serde::Deserialize;
use std::{fmt::Display, str::FromStr};

/// Parse the first path parameter found in the API Gateway request, into the provided type.
///
/// ```rust
/// use effortless::api::parse_path_parameter;
/// use lambda_http::{Request, RequestExt};
///
/// use std::collections::HashMap;
///
/// let event = Request::default()
///   .with_path_parameters(HashMap::from([("id".to_string(), "1".to_string())]))
///   .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
///     lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
/// ));
/// let id_param = parse_path_parameter::<i32>(&event, "id").unwrap();
/// assert_eq!(id_param, Some(1));
/// ```
pub fn parse_path_parameter<T>(event: &Request, parameter: &str) -> Result<Option<T>, APIErrors>
where
    T: FromStr,
    T::Err: Display,
{
    match fragment::parse_path_parameter::<T>(event, parameter) {
        Some(value) => match value {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                let api_error = APIError::with_pointer(
                    get_apigw_request_id(event),
                    parameter,
                    format!("invalid path parameter `{parameter}`: {e}").as_str(),
                );
                Err(APIErrors::new(&[api_error]))
            }
        },
        None => Ok(None),
    }
}

/// Parse the first path parameter found in the API Gateway request, into the provided type.
///
/// ```rust
/// use effortless::api::parse_query_string_parameter;
/// use lambda_http::{Request, RequestExt};
///
/// use std::collections::HashMap;
///
/// let event = Request::default()
///   .with_query_string_parameters(HashMap::from([("status".to_string(), "Pending".to_string())]))
///   .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
///     lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
/// ));
/// let id_param = parse_query_string_parameter::<String>(&event, "status").unwrap();
/// assert_eq!(id_param, Some("Pending".to_string()));
/// ```
pub fn parse_query_string_parameter<T>(
    event: &Request,
    parameter: &str,
) -> Result<Option<T>, APIErrors>
where
    T: FromStr,
    T::Err: Display,
{
    match fragment::parse_query_string_parameter::<T>(event, parameter) {
        Some(value) => match value {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                let api_error = APIError::with_parameter(
                    get_apigw_request_id(event),
                    parameter,
                    format!("invalid query string parameter `{parameter}`: {e}").as_str(),
                );
                Err(APIErrors::new(&[api_error]))
            }
        },
        None => Ok(None),
    }
}

/// Parse the request body.
///
/// ```rust
/// use effortless::api::parse_request_body;
/// use lambda_http::{Request, RequestExt};
///
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// pub struct Person{
///   pub first_name: String,
///   pub last_name: String
/// }
///
/// let event = Request::new("{\n  \"first_name\": \"Rosa\",\n  \"last_name\": \"Maria\"\n}".into())
///   .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
///     lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
/// ));
/// let person = parse_request_body::<Person>(&event).unwrap();
/// assert_eq!(person.first_name, "Rosa");
/// assert_eq!(person.last_name, "Maria");
/// ```
pub fn parse_request_body<T>(event: &Request) -> Result<T, APIErrors>
where
    T: for<'a> Deserialize<'a>,
{
    match fragment::parse_request_body::<T>(event) {
        Ok(o) => Ok(o),
        Err(e) => {
            let api_error = APIError::new(
                get_apigw_request_id(event),
                StatusCode::BAD_REQUEST,
                "Invalid Body",
                e.to_string().as_str(),
                None,
            );
            Err(APIErrors::new(&[api_error]))
        }
    }
}

/// Create an APIError representing an item not found error.
pub fn entry_not_found(event: &Request) -> APIError {
    APIError::new(
        get_apigw_request_id(event),
        StatusCode::NOT_FOUND,
        "Item Not Found",
        "the entry was not found",
        event
            .uri()
            .path_and_query()
            .map(|p| APIErrorSource::Parameter(p.to_string())),
    )
}

/// Create an APIError from an API Gateway event, indicating that a parameter was missing.
pub fn missing_parameter(event: &Request, parameter: &str) -> APIError {
    APIError::new(
        get_apigw_request_id(event),
        StatusCode::BAD_REQUEST,
        "Missing Parameter",
        format!("parameter {parameter} was not provided").as_str(),
        event
            .uri()
            .path_and_query()
            .map(|p| APIErrorSource::Parameter(p.to_string())),
    )
}

/// Create an APIError from an API Gateway event, representing an internal error.
pub fn internal_error(event: &Request, details: &str) -> APIError {
    APIError::new(
        get_apigw_request_id(event),
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error",
        details,
        event
            .uri()
            .path_and_query()
            .map(|p| APIErrorSource::Parameter(p.to_string())),
    )
}

/// Create an APIError from an API Gateway event, representing an invalid body issue.
pub fn invalid_body(event: &Request, details: &str) -> APIError {
    APIError::new(
        get_apigw_request_id(event),
        StatusCode::BAD_REQUEST,
        "Invalid Body",
        details,
        None,
    )
}
