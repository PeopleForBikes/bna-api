pub mod link_header;

use bnacore::aws::get_aws_secrets_value;
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use tracing::{debug, error};

/// The result type to return to the caller of the Lambda API handler.
pub type APIResult<T> = std::result::Result<T, Response<Body>>;

/// Maximum number of items allowed to be returned by a query at once.
pub const MAX_PAGE_SIZE: u64 = 100;
/// Number of items to return per page if no argument was provided.
pub const DEFAULT_PAGE_SIZE: u64 = 50;

/// Returns the database connection.
///
/// Look up for the connection string:
///   - first inside the value of the `DATABASE_URL`environment variable,
///   - then in the AWS Secrets Manager with the `secret_id` id if provided.
pub async fn database_connect(secret_id: Option<&str>) -> Result<DatabaseConnection, DbErr> {
    const DATABASE_URL_KEY: &str = "DATABASE_URL";
    let database_url: String = match env::var(DATABASE_URL_KEY) {
        Ok(value) => value,
        Err(_) => match secret_id {
            Some(secret_id) =>
                match env::var(secret_id) {
                  Ok(v) => get_aws_secrets_value(&v, DATABASE_URL_KEY).await.map_err(|e| DbErr::Custom(format!("Cannot find the connection string within the secret {secret_id}. Ensure `{DATABASE_URL_KEY}` is correctly set: {e}")))?,
                  Err(e) => return Err(DbErr::Custom(format!("Cannot find the connection string in the AWS Secrets Manager. Ensure `{secret_id}` is correctly set. Reason: {e}"))),
            }
            None => return Err(DbErr::Custom(format!("Cannot find the connection string. Ensure `{DATABASE_URL_KEY}` is correctly set."))),
        },
    };
    Database::connect(database_url).await
}

/// Retrieves the pagination parameters.
///
/// If nothing is provided, the first page is returned and will contain up to
/// [`DEFAULT_PAGE_SIZE`] items.
///
/// If `page` does not exist, the lambda functions will return an empty array.
pub fn pagination_parameters(event: &Request) -> APIResult<(u64, u64)> {
    debug!("Retrieving pagination...");
    let apigw_request_id = get_apigw_request_id(event);
    let page_size = match event
        .query_string_parameters()
        .first("page_size")
        .unwrap_or(DEFAULT_PAGE_SIZE.to_string().as_str())
        .parse::<u64>()
    {
        Ok(page_size) => match page_size {
            0 => 1,
            1..=MAX_PAGE_SIZE => page_size,
            _ => MAX_PAGE_SIZE,
        },
        Err(e) => {
            let api_error = APIError::with_parameter(
                apigw_request_id,
                "page_size",
                format!("Failed to process the page_size parameter: {e}").as_str(),
            );
            return Err(APIErrors::new(&[api_error]).into());
        }
    };
    let page = match event
        .query_string_parameters()
        .first("page")
        .unwrap_or("1")
        .parse::<u64>()
    {
        Ok(page) => match page {
            0 => 1,
            _ => page,
        },
        Err(e) => {
            let api_error = APIError::with_parameter(
                apigw_request_id,
                "page",
                format!("Failed to process the page parameter: {e}").as_str(),
            );
            return Err(APIErrors::new(&[api_error]).into());
        }
    };

    Ok((page_size, page))
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

/// Builds a paginated Response.
///
/// Builds a Response struct which contains the pagination information in the headers.
///
/// Implements pagination headers, similar to GitLab:
/// <https://docs.gitlab.com/ee/api/rest/index.html#other-pagination-headers>
///
/// - x-next-page   The index of the next page.
/// - x-page        The index of the current page (starting at 1).
/// - x-per-page    The number of items per page.
/// - x-prev-page   The index of the previous page.
/// - x-total       The total number of items.
/// - x-total-pages The total number of pages.
///
/// The link header is also added if the URL is provided by the request event.
pub fn build_paginated_response(
    body: Value,
    total_items: u64,
    page: u64,
    page_size: u64,
    event: &Request,
) -> Result<Response<Body>, Error> {
    let paginatron = Paginatron::new(event.uri().host(), total_items, page, page_size);
    let nav = paginatron.navigation();

    // Build the response.
    Ok(Response::builder()
        .header("link", paginatron.link_header(Some(DEFAULT_PAGE_SIZE)))
        .header("x-next-page", nav.next())
        .header("x-page", page)
        .header("x-per-page", page_size)
        .header("x-prev-page", nav.prev())
        .header("x-total", total_items)
        .header("x-total-pages", nav.last())
        .body(body.to_string().into())
        .map_err(Box::new)?)
}

/// Stores the page values required for pagination.
pub struct NavigationPages {
    first: u64,
    prev: u64,
    next: u64,
    last: u64,
}

impl NavigationPages {
    /// Creates a new NavigationPages.
    pub fn new(first: u64, prev: u64, next: u64, last: u64) -> Self {
        Self {
            first,
            prev,
            next,
            last,
        }
    }

    /// Returns the first page number.
    pub fn first(&self) -> u64 {
        self.first
    }

    /// Returns the previous page number.
    pub fn prev(&self) -> u64 {
        self.prev
    }

    /// Returns the next page number.
    pub fn next(&self) -> u64 {
        self.next
    }

    /// Returns the last page number.
    pub fn last(&self) -> u64 {
        self.last
    }
}

/// Generates the pagination information.
pub struct Paginatron<'a> {
    url: Option<&'a str>,
    total_items: u64,
    page: u64,
    page_size: u64,
}

impl<'a> Paginatron<'a> {
    /// Creates a new Paginatron.
    pub fn new(url: Option<&'a str>, total_items: u64, page: u64, page_size: u64) -> Self {
        Self {
            url,
            total_items,
            page,
            page_size,
        }
    }

    /// Prepares the pagination values.
    ///
    /// ```
    /// use lambdas::Paginatron;
    ///
    /// let paginatron = Paginatron::new(None, 200, 3, 25);
    /// let nav = paginatron.navigation();
    ///
    /// assert_eq!(nav.first(), 1);
    /// assert_eq!(nav.prev(), 2);
    /// assert_eq!(nav.next(), 4);
    /// assert_eq!(nav.last(), 8);
    /// ```
    pub fn navigation(&self) -> NavigationPages {
        const FIRST: u64 = 1;
        let last = (self.total_items + self.page_size - 1) / self.page_size;
        let page = if self.page >= last { last } else { self.page };
        let previous = if page <= FIRST { FIRST } else { page - 1 };
        let next = if page >= last { last } else { page + 1 };
        NavigationPages::new(FIRST, previous, next, last)
    }

    /// Generates the link header.
    ///
    /// If the url is not available, the link will be an empty string.
    ///
    /// If the default page size is provided and is different from the paginator
    /// page size, the page_size query parameter will be omitted.
    ///
    /// ```
    /// use lambdas::Paginatron;
    ///
    /// let paginatron = Paginatron::new(Some("https://api.peopleforbikes.xyz/bnas"), 12875, 3, 25);
    /// assert_eq!(
    ///   paginatron.link_header(Some(25)),
    ///   r#"<https://api.peopleforbikes.xyz/bnas?page=1>; rel="first", <https://api.peopleforbikes.xyz/bnas?page=2>; rel="prev", <https://api.peopleforbikes.xyz/bnas?page=4>; rel="next", <https://api.peopleforbikes.xyz/bnas?page=515>; rel="last""#
    /// );
    /// ```
    ///
    pub fn link_header(&self, defaul_page_size: Option<u64>) -> String {
        match self.url {
            None => String::new(),
            Some(url) => {
                let nav = self.navigation();
                let mut url = url.to_string();
                url.push('?');
                if let Some(default_page_size) = defaul_page_size {
                    if self.page_size != default_page_size {
                        url.push_str(format!("page_size={}&", self.page_size).as_str());
                    }
                }
                let first = format!(r#"<{}page={}>; rel="first""#, url, nav.first());
                let prev = format!(r#"<{}page={}>; rel="prev""#, url, nav.prev());
                let next = format!(r#"<{}page={}>; rel="next""#, url, nav.next());
                let last = format!(r#"<{}page={}>; rel="last""#, url, nav.last());
                format!("{first}, {prev}, {next}, {last}")
            }
        }
    }
}

/// Generates a TryFrom<&'a str> implementation for a specific type.
///
/// This macros expects the following items to be setup before use:
///   - the type must implement a `parse` function with the following signature
///       `pub fn parse(i: &str) -> IResult<&str, __type__>`
#[macro_export]
macro_rules! nomstr {
    ($a:ident) => {
        impl<'a> TryFrom<&'a str> for $a<'a> {
            type Error = Error<String>;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                match $a::parse(value).finish() {
                    Ok((_, item)) => Ok(item),
                    Err(Error { input, code }) => Err(Error {
                        input: input.to_string(),
                        code,
                    }),
                }
            }
        }
    };
}

/// An object containing references to the primary source of the error.
///
/// It SHOULD include one of the following members or be omitted:
///
///   - pointer: a JSON Pointer [RFC6901](https://tools.ietf.org/html/rfc6901) to the
///     value in the request document that caused the error [e.g. "/data" for a primary
///     data object, or "/data/attributes/title" for a specific attribute].
///     This MUST point to a value in the request document that exists; if it doesn’t,
///     the client SHOULD simply ignore the pointer.
///   - parameter: a string indicating which URI query parameter caused the error.
///   - header: a string indicating the name of a single request header which caused the
///     error.
#[derive(Deserialize, Serialize, Clone)]
pub enum APIErrorSource {
    Pointer(String),
    Parameter(String),
    Header(String),
}

/// Single API Error object as described in <https://jsonapi.org/format/#error-objects>.
#[derive(Deserialize, Serialize, Clone)]
pub struct APIError {
    id: Option<String>,
    // Cannot use http_serde 2.0.0 until lambda_http upgraded the http crate to 1.0.0.
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    title: String,
    details: String,
    source: APIErrorSource,
}

impl APIError {
    /// Creates a new `APIError`.
    pub fn new(
        id: Option<String>,
        status: StatusCode,
        title: String,
        details: String,
        source: APIErrorSource,
    ) -> Self {
        Self {
            id,
            status,
            title,
            details,
            source,
        }
    }

    /// Creates a new `APIError` based off a query parameter error.
    pub fn with_parameter(id: Option<String>, parameter: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::BAD_REQUEST,
            title: String::from("Invalid Query Parameter"),
            source: APIErrorSource::Parameter(parameter.into()),
            details: message.into(),
        }
    }

    /// Creates a new `APIError` based off an invalid attribute.
    pub fn with_pointer(id: Option<String>, pointer: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::BAD_REQUEST,
            title: String::from("Invalid Attribute"),
            source: APIErrorSource::Pointer(pointer.into()),
            details: message.into(),
        }
    }

    // Creates a new `APIError` for internal errors.
    pub fn internal_error(id: Option<String>, title: &str, details: &str, source: &str) -> Self {
        Self {
            id,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            source: APIErrorSource::Pointer(source.into()),
            title: title.into(),
            details: details.into(),
        }
    }

    /// Creates a new `APIError` for database issues.
    pub fn db_error(id: Option<String>, source: &str, message: &str) -> Self {
        APIError::internal_error(id, "Database error", message, source)
    }

    /// Creates a new `APIError` for no-content issues.
    pub fn no_content(id: Option<String>, source: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::NOT_FOUND,
            title: String::from("Content Not Found"),
            source: APIErrorSource::Pointer(source.into()),
            details: message.into(),
        }
    }
}

impl From<APIError> for Response<Body> {
    fn from(value: APIError) -> Self {
        Response::builder()
            .status(value.status)
            .body(json!(value).to_string().into())
            .unwrap()
    }
}

/// Error objects MUST be returned as an array keyed by errors in the top level of a
/// JSON:API document.
#[derive(Deserialize, Serialize)]
pub struct APIErrors {
    errors: Vec<APIError>,
}

impl APIErrors {
    /// Creates a new `APIErrors`.
    pub fn new(errors: &[APIError]) -> Self {
        Self {
            errors: errors.to_vec(),
        }
    }
}

impl From<APIErrors> for Response<Body> {
    /// Converts this object to a `Response<Body>`.
    ///
    /// If there is only one error returned, the Response status code will be the same
    /// as the one of the error. Otherwise it will be set to [StatusCode::BAD_REQUEST].
    fn from(value: APIErrors) -> Self {
        let status = if value.errors.len() == 1 {
            value.errors.first().unwrap().status
        } else {
            StatusCode::BAD_REQUEST
        };
        Response::builder()
            .status(status)
            .body(json!(value).to_string().into())
            .unwrap()
    }
}

pub async fn api_database_connect(event: &Request) -> APIResult<DatabaseConnection> {
    debug!("Connecting to the database...");
    match database_connect(Some("DATABASE_URL_SECRET_ID")).await {
        Ok(db) => Ok(db),
        Err(e) => {
            let apigw_request_id = get_apigw_request_id(event);
            let error_msg = "cannot connect to the database".to_string();
            error!(
                "{error_msg}: {e}. API GW Request ID: {:?}",
                apigw_request_id
            );
            let api_error = APIError::db_error(apigw_request_id, event.uri().path(), &error_msg);
            Err(APIErrors::new(&[api_error]).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::request::from_str;
    use std::collections::HashMap;

    #[test]
    fn test_paginatron_with_different_page_size() {
        let paginatron = Paginatron::new(Some("https://api.peopleforbikes.xyz/bnas"), 200, 3, 25);
        assert_eq!(
            paginatron.link_header(Some(50)),
            r#"<https://api.peopleforbikes.xyz/bnas?page_size=25&page=1>; rel="first", <https://api.peopleforbikes.xyz/bnas?page_size=25&page=2>; rel="prev", <https://api.peopleforbikes.xyz/bnas?page_size=25&page=4>; rel="next", <https://api.peopleforbikes.xyz/bnas?page_size=25&page=8>; rel="last""#
        );
    }

    #[test]
    fn test_paginatron_invalid_page() {
        let paginatron = Paginatron::new(Some("https://api.peopleforbikes.xyz/bnas"), 42, 3, 25);
        let nav = paginatron.navigation();
        assert_eq!(nav.first(), 1);
        assert_eq!(nav.prev(), 1);
        assert_eq!(nav.next(), 2);
        assert_eq!(nav.last(), 2);
    }

    #[test]
    fn test_pagination_parameters_without_params() {
        let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
        let req = from_str(input).unwrap();

        let actual = pagination_parameters(&req).unwrap();
        assert_eq!(actual.0, DEFAULT_PAGE_SIZE);
        assert_eq!(actual.1, 1);
    }

    #[test]
    fn test_pagination_parameters_with_valid_params() {
        const PAGE_SIZE: u64 = 25;
        const PAGE: u64 = 8;

        let mut data = HashMap::new();
        data.insert("page_size".into(), vec![PAGE_SIZE.to_string()]);
        data.insert("page".into(), vec![PAGE.to_string()]);

        let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
        let result = from_str(input).unwrap();
        let req = result.with_query_string_parameters(data);

        let actual = pagination_parameters(&req).unwrap();
        assert_eq!(actual.0, PAGE_SIZE);
        assert_eq!(actual.1, PAGE);
    }

    #[test]
    fn test_pagination_parameters_with_invalid_page_size() {
        let mut data = HashMap::new();
        data.insert("page_size".into(), vec!["-1".to_string()]);
        data.insert("page".into(), vec!["50".to_string()]);

        let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
        let result = from_str(input).unwrap();
        let req = result.with_query_string_parameters(data);

        let actual = pagination_parameters(&req).unwrap_err();

        // Ensure the error had the BAD_REQUEST status.
        assert_eq!(actual.status(), StatusCode::BAD_REQUEST);

        // Ensure the error message is correct.
        let b = actual.body();
        let message = match b {
            Body::Text(message) => message,
            _ => panic!("The body does not match the Text invariant."),
        };
        let api_error: APIErrors = serde_json::from_str(message).unwrap();
        assert_eq!(api_error.errors.len(), 1)
    }

    #[test]
    fn test_pagination_parameters_with_invalid_page() {
        let mut data = HashMap::new();
        data.insert("page_size".into(), vec!["1".to_string()]);
        data.insert("page".into(), vec!["abc".to_string()]);

        let input = include_str!("fixtures/api-gateway-v2-proxy-request-minimal.json");
        let result = from_str(input).unwrap();
        let req = result.with_query_string_parameters(data);

        let actual = pagination_parameters(&req).unwrap_err();

        // Ensure the error had the BAD_REQUEST status.
        assert_eq!(actual.status(), StatusCode::BAD_REQUEST);

        // Ensure the error message is correct.
        let b = actual.body();
        let message = match b {
            Body::Text(message) => message,
            _ => panic!("The body does not match the Text invariant."),
        };
        let api_error: APIErrors = serde_json::from_str(message).unwrap();
        assert_eq!(api_error.errors.len(), 1)
    }
}
