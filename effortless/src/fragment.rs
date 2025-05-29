use lambda_http::{aws_lambda_events::query_map::QueryMap, http, Request, RequestExt};
use serde::Deserialize;
use std::{str::FromStr, string::FromUtf8Error};
use thiserror::Error;
use urlencoding::decode;

#[derive(Error, Debug)]
pub enum ParseParameterError {
    #[error("parsing error: {0}")]
    ParseError(String),
    #[error("encoding error: {0}")]
    URLEncodingError(#[from] FromUtf8Error),
}

/// Parse a parameter found in a QueryMap.
fn parse_parameter<T>(qm: &QueryMap, parameter: &str) -> Option<Result<T, ParseParameterError>>
where
    T: FromStr,
{
    qm.first(parameter)
        .map(|parameter_str| match decode(parameter_str) {
            Ok(param) => match param.parse::<T>() {
                Ok(param) => Some(Ok(param)),
                Err(_) => Some(Err(ParseParameterError::ParseError(format!(
                    "cannot parse parameter {parameter_str}"
                )))),
            },
            Err(e) => Some(Err(e.into())),
        })?
}

/// Parse the first matching path parameter into the provided type.
///
/// Returns `None` if the parameter does not exist, `Some(Err)` if the parsing failed,
/// or Some(Ok(T)) if the parsing succeeded.
///
/// ```rust
/// use effortless::fragment::parse_path_parameter;
/// use lambda_http::{Request, RequestExt};
/// use std::collections::HashMap;
///
/// let event = Request::default()
///   .with_path_parameters(HashMap::from([("id".to_string(), "1".to_string())]));
/// let param = parse_path_parameter::<i32>(&event, "id").unwrap();
/// assert_eq!(param.unwrap(), 1);
/// ```
pub fn parse_path_parameter<T>(
    event: &Request,
    parameter: &str,
) -> Option<Result<T, ParseParameterError>>
where
    T: FromStr,
{
    parse_parameter(&event.path_parameters(), parameter)
}

/// Parse the first matching query string parameter into the provided type.
///
/// ```rust
/// use effortless::fragment::parse_query_string_parameter;
/// use lambda_http::{Request, RequestExt};
/// use std::collections::HashMap;
/// use serde::Deserialize;
///
/// let event = Request::default()
///   .with_query_string_parameters(HashMap::from([("status".to_string(), "Pending".to_string())]));
/// let param = parse_query_string_parameter::<String>(&event, "status").unwrap();
/// assert_eq!(param.unwrap(), "Pending".to_string());
/// ```
pub fn parse_query_string_parameter<T>(
    event: &Request,
    parameter: &str,
) -> Option<Result<T, ParseParameterError>>
where
    T: FromStr,
{
    parse_parameter(&event.query_string_parameters(), parameter)
}

/// Parse the request body.
///
/// ```rust
/// use effortless::fragment::parse_request_body;
/// use lambda_http::{Request, RequestExt};
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// pub struct Person{
///   pub first_name: String,
///   pub last_name: String
/// }
///
/// let event = Request::new("{\n  \"first_name\": \"Rosa\",\n  \"last_name\": \"Maria\"\n}".into());
/// let person = parse_request_body::<Person>(&event).unwrap();
/// assert_eq!(person.first_name, "Rosa");
/// assert_eq!(person.last_name, "Maria");
/// ```
pub fn parse_request_body<T>(event: &Request) -> Result<T, std::io::Error>
where
    T: for<'a> Deserialize<'a>,
{
    let body_str = std::str::from_utf8(event.body()).map_err(std::io::Error::other)?;
    // serde_json::Error::custom("body contains invalid UTF-8 characters"))?;
    Ok(serde_json::from_str::<T>(body_str)?)
}

// Returns the Api Gateway Request ID from an ApiGatewayV2 event.
//
// If there is no request ID or the event is not coming from an ApiGatewayV2, the
// function returns None.
pub fn get_apigw_request_id(event: &Request) -> Option<String> {
    match event.request_context() {
        lambda_http::request::RequestContext::ApiGatewayV2(payload) => payload.request_id,
        _ => None,
    }
}

/// Attempt to create an extension trait for [`lambda_http::Request`].
pub trait BnaRequestExt {
    /// Return the first matching parameter from the QueryMap, deserialized into its type T, if it exists.
    fn parse_parameter<T>(qm: &QueryMap, parameter: &str) -> Option<Result<T, <T as FromStr>::Err>>
    where
        T: FromStr,
    {
        qm.first(parameter)
            .map(|parameter_str| match parameter_str.parse::<T>() {
                Ok(param) => Some(Ok(param)),
                Err(e) => Some(Err(e)),
            })?
    }

    /// Return the first matching path parameter if it exists.
    fn first_path_parameter(&self, parameter: &str) -> Option<String>;

    /// Return the first matching query string parameter if it exists.
    fn first_query_string_parameter(&self, parameter: &str) -> Option<String>;

    /// Return the first matching path parameter deserialized into its type T, if it exists.
    fn path_parameter<T>(&self, parameter: &str) -> Option<Result<T, T::Err>>
    where
        T: FromStr;

    /// Return the first matching path parameter deserialized into its type T, if it exists.
    fn query_string_parameter<T>(&self, parameter: &str) -> Option<Result<T, T::Err>>
    where
        T: FromStr;

    /// Returns the Api Gateway Request ID from an ApiGatewayV2 event.
    ///
    /// If there is no request ID or the event is not coming from an ApiGatewayV2, the
    /// function returns None.
    fn apigw_request_id(&self) -> Option<String>;

    /// Returns true if there are path parameters available.
    fn has_path_parameters(&self) -> bool;

    /// Returns true if there are query parameters available.
    fn has_query_parameters(&self) -> bool;

    /// Returns the path and query or "/" if not available.
    fn path_and_query(&self) -> String;
}

impl<B> BnaRequestExt for http::Request<B> {
    fn first_path_parameter(&self, parameter: &str) -> Option<String> {
        self.path_parameters().first(parameter).map(String::from)
    }

    fn first_query_string_parameter(&self, parameter: &str) -> Option<String> {
        self.query_string_parameters()
            .first(parameter)
            .map(String::from)
    }

    fn path_parameter<T>(&self, parameter: &str) -> Option<Result<T, <T as FromStr>::Err>>
    where
        T: FromStr,
    {
        Self::parse_parameter::<T>(&self.path_parameters(), parameter)
    }

    fn query_string_parameter<T>(&self, parameter: &str) -> Option<Result<T, <T as FromStr>::Err>>
    where
        T: FromStr,
    {
        Self::parse_parameter::<T>(&self.query_string_parameters(), parameter)
    }

    fn apigw_request_id(&self) -> Option<String> {
        match self.request_context() {
            lambda_http::request::RequestContext::ApiGatewayV2(payload) => payload.request_id,
            _ => None,
        }
    }

    fn has_path_parameters(&self) -> bool {
        !self.path_parameters().is_empty()
    }

    fn has_query_parameters(&self) -> bool {
        !self.query_string_parameters().is_empty()
    }

    fn path_and_query(&self) -> String {
        self.uri()
            .path_and_query()
            .map_or(String::from("/"), |p| p.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    #[test]
    fn test_url_decode_path_parameter() {
        let event = Request::default().with_path_parameters(HashMap::from([(
            "country".to_string(),
            "United%20States".to_string(),
        )]));
        let param = parse_path_parameter::<String>(&event, "country").unwrap();
        assert_eq!(param.unwrap(), String::from("United States"));
    }
}
