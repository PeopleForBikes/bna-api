pub mod link_header;

use bnacore::aws::get_aws_secrets_value;
use effortless::{
    error::{APIError, APIErrors},
    fragment::get_apigw_request_id,
};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde_json::Value;
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
    use aws_lambda_events::http;
    use effortless::api::{parse_path_parameter, parse_request_body};
    use entity::wrappers::submission::SubmissionPost;
    use lambda_http::{http::StatusCode, request::from_str, RequestExt};
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

    #[test]
    fn test_parse_path_parameter() {
        let event = Request::default()
            .with_path_parameters(HashMap::from([("id".to_string(), "1".to_string())]));
        let p = parse_path_parameter::<i32>(&event, "id");
        assert_eq!(p.unwrap(), Some(1));
    }

    #[test]
    fn test_apigw_parse_path_parameter() {
        let event = Request::default()
            .with_path_parameters(HashMap::from([("id".to_string(), "1".to_string())]))
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let id_param = parse_path_parameter::<i32>(&event, "id").unwrap();
        assert_eq!(id_param, Some(1));
    }

    #[test]
    fn test_parse_request_body() {
        let event = http::Request::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"city": "santa rosa","country": "usa","email": "jane.dpe@orgllc.com","fips_code": "3570670","first_name": "Jane","last_name": "Doe","organization": "Organization LLC","region": "new mexico","title": "CTO","consent": true}"#))
            .expect("failed to build request");
        let submission = parse_request_body::<SubmissionPost>(&event).unwrap();
        assert_eq!(submission.country, "usa")
    }

    #[test]
    fn test_apigw_parse_request_body() {
        let event = http::Request::builder()
      .header(http::header::CONTENT_TYPE, "application/json")
      .body(Body::from(r#"{"city": "santa rosa","country": "usa","email": "jane.dpe@orgllc.com","fips_code": "3570670","first_name": "Jane","last_name": "Doe","organization": "Organization LLC","region": "new mexico","title": "CTO","consent": true}"#))
      .expect("failed to build request");
        let submission = parse_request_body::<SubmissionPost>(&event).unwrap();
        assert_eq!(submission.country, "usa")
    }
}
