use crate::{
    error::{APIError, APIErrorSource, APIErrors},
    fragment::{self, get_apigw_request_id},
};
use lambda_http::{http::StatusCode, Request, RequestPayloadExt};
use serde::de::DeserializeOwned;
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
/// use lambda_http::{http::{self, StatusCode}, Body, Request, RequestExt};
///
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// pub struct Person{
///   pub first_name: String,
///   pub last_name: String
/// }
///
/// let event = http::Request::builder()
///   .header(http::header::CONTENT_TYPE, "application/json")
///   .body(Body::from(r#"{"first_name": "Rosa","last_name": "Maria"}"#))
///   .expect("failed to build request");
/// let person = parse_request_body::<Person>(&event).unwrap();
/// assert_eq!(person.first_name, "Rosa");
/// assert_eq!(person.last_name, "Maria");
/// ```
pub fn parse_request_body<T>(event: &Request) -> Result<T, APIErrors>
where
    T: DeserializeOwned,
{
    let payload = event
        .payload::<T>()
        .map_err(|e| invalid_body(event, e.to_string().as_str()))?;
    Ok(payload.ok_or_else(|| invalid_body(event, "No request body was provided."))?)
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

/// Create and APIError from and API Gateway event, representing a path parameter issue.
pub fn invalid_path_parameter(event: &Request, parameter: &str, details: &str) -> APIError {
    APIError::with_pointer(
        get_apigw_request_id(event),
        parameter,
        format!("invalid path parameter `{parameter}`: {details}").as_str(),
    )
}

/// Create and APIError from and API Gateway event, representing a query parameter issue.
pub fn invalid_query_parameter(event: &Request, parameter: &str, details: &str) -> APIError {
    APIError::with_pointer(
        get_apigw_request_id(event),
        parameter,
        format!("invalid query parameter `{parameter}`: {details}").as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::APIErrorSource::Parameter;
    use lambda_http::{
        http::{self, header, StatusCode},
        Body, RequestExt,
    };
    use std::collections::HashMap;

    // #[test]
    // fn test_pagination_parameters_without_params() {
    //     let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
    //     let req = from_str(input).unwrap();

    //     let actual = extract_pagination_parameters(&req).unwrap();
    //     assert_eq!(actual.page_size(), DEFAULT_PAGE_SIZE);
    //     assert_eq!(actual.page(), 0);
    // }

    // #[test]
    // fn test_pagination_parameters_with_valid_params() {
    //     const PAGE_SIZE: u64 = 25;
    //     const PAGE: u64 = 8;

    //     let mut data = HashMap::new();
    //     data.insert("page_size".into(), vec![PAGE_SIZE.to_string()]);
    //     data.insert("page".into(), vec![PAGE.to_string()]);

    //     let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
    //     let result = from_str(input).unwrap();
    //     let req = result.with_query_string_parameters(data);

    //     let actual = extract_pagination_parameters(&req).unwrap();
    //     assert_eq!(actual.page_size(), PAGE_SIZE);
    //     assert_eq!(actual.page(), PAGE);
    // }

    // #[test]
    // fn test_pagination_parameters_with_invalid_page_size() {
    //     let mut data = HashMap::new();
    //     data.insert("page_size".into(), vec!["-1".to_string()]);
    //     data.insert("page".into(), vec!["50".to_string()]);

    //     let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
    //     let result = from_str(input).unwrap();
    //     let req = result.with_query_string_parameters(data);

    //     let actual = extract_pagination_parameters(&req).unwrap_err();

    //     // Ensure the error had the BAD_REQUEST status.
    //     assert_eq!(actual.status(), StatusCode::BAD_REQUEST);

    //     // Ensure the error message is correct.
    //     let b = actual.body();
    //     let message = match b {
    //         Body::Text(message) => message,
    //         _ => panic!("The body does not match the Text invariant."),
    //     };
    //     let api_error: APIErrors = serde_json::from_str(message).unwrap();
    //     assert_eq!(api_error.errors.len(), 1)
    // }

    // #[test]
    // fn test_pagination_parameters_with_invalid_page() {
    //     let mut data = HashMap::new();
    //     data.insert("page_size".into(), vec!["1".to_string()]);
    //     data.insert("page".into(), vec!["abc".to_string()]);

    //     let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
    //     let result = from_str(input).unwrap();
    //     let req = result.with_query_string_parameters(data);

    //     let actual = extract_pagination_parameters(&req).unwrap_err();

    //     // Ensure the error had the BAD_REQUEST status.
    //     assert_eq!(actual.status(), StatusCode::BAD_REQUEST);

    //     // Ensure the error message is correct.
    //     let b = actual.body();
    //     let message = match b {
    //         Body::Text(message) => message,
    //         _ => panic!("The body does not match the Text invariant."),
    //     };
    //     let api_error: APIErrors = serde_json::from_str(message).unwrap();
    //     assert_eq!(api_error.errors.len(), 1)
    // }

    #[test]
    fn test_entry_not_found() {
        let event = http::Request::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .uri("/bnas/eac1dbfb-2137-44c5-be59-71fc613f2963")
            .body(Body::Empty)
            .expect("failed to build request")
            .with_path_parameters(HashMap::from([(
                "rating_id".to_string(),
                "eac1dbfb-2137-44c5-be59-71fc613f2963".to_string(),
            )]))
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let actual = entry_not_found(&event);
        let expected = APIError::new(
            None,
            StatusCode::NOT_FOUND,
            "Item Not Found",
            "the entry was not found",
            Some(Parameter(
                "/bnas/eac1dbfb-2137-44c5-be59-71fc613f2963".to_string(),
            )),
        );
        assert_eq!(actual, expected);
    }
}
