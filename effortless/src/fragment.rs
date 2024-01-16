use lambda_http::{aws_lambda_events::query_map::QueryMap, Request, RequestExt};
use serde::Deserialize;
use std::str::FromStr;

fn parse_parameter<T>(qm: &QueryMap, parameter: &str) -> Option<Result<T, <T as FromStr>::Err>>
where
    T: FromStr,
{
    match qm.first(parameter) {
        Some(parameter_str) => {
            let parsed_parameter = parameter_str.parse::<T>();
            match parsed_parameter {
                Ok(param) => Some(Ok(param)),
                Err(e) => Some(Err(e)),
            }
        }
        None => None,
    }
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
) -> Option<Result<T, <T as FromStr>::Err>>
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
) -> Option<Result<T, <T as FromStr>::Err>>
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
    let body_str = std::str::from_utf8(event.body())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
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
