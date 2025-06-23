pub mod core;

use axum::{
    extract::{FromRequest, OriginalUri},
    response::IntoResponse,
};
use bnacore::aws::get_aws_secrets_value;
use effortless::{
    api::DEFAULT_PAGE_SIZE,
    error::{APIError, APIErrors},
    fragment::BnaRequestExt,
};
use lambda_http::{
    http::{header, StatusCode},
    Body, Error, Request, Response,
};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_plain::derive_fromstr_from_deserialize;
use std::env;
use tracing::{debug, error};

/// The result type to return to the caller of the Lambda API handler.
pub type APIResult<T> = std::result::Result<T, Response<Body>>;

// /// Maximum number of items allowed to be returned by a query at once.
// pub const MAX_PAGE_SIZE: u64 = 100;
// /// Number of items to return per page if no argument was provided.
// pub const DEFAULT_PAGE_SIZE: u64 = 50;

/// Returns the database connection from a custom environment variable or secret.
///
/// Look up for the connection string:
///   - first inside the value of the `name` environment variable,
///   - then in the AWS Secrets Manager with the `name`  and  `secret_key` provided.
pub async fn database_connect_custom(
    name: &str,
    secret_key: &str,
) -> Result<DatabaseConnection, DbErr> {
    let database_url: String = match env::var(name) {
        Ok(value) => value,
        Err(_) => get_aws_secrets_value(name, secret_key).await.map_err(|e| DbErr::Custom(format!("Cannot find the connection string within the secret {name}. Ensure `{secret_key}` is correctly set: {e}")))?,
    };
    Database::connect(database_url).await
}

/// Returns the database connection.
///
/// Look up for the database connection string a standard `DATABASE_URL` environment
/// variable or secret.
pub async fn database_connect() -> Result<DatabaseConnection, DbErr> {
    const DATABASE_URL_KEY: &str = "DATABASE_URL";
    database_connect_custom(DATABASE_URL_KEY, DATABASE_URL_KEY).await
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
    let paginatron = Paginatron::new(
        event.uri().host().map(|h| h.to_string()),
        total_items,
        page,
        page_size,
    );
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
        .header(header::CONTENT_TYPE, "application/json")
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
#[derive(Serialize)]
pub struct Paginatron {
    url: Option<String>,
    total_items: u64,
    page: u64,
    page_size: u64,
}

impl Paginatron {
    /// Creates a new Paginatron.
    pub fn new(url: Option<String>, total_items: u64, page: u64, page_size: u64) -> Self {
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
        let last = self.total_items.div_ceil(self.page_size);
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
    /// let paginatron = Paginatron::new(Some("https://api.peopleforbikes.xyz/bnas".to_string()), 12875, 3, 25);
    /// assert_eq!(
    ///   paginatron.link_header(Some(25)),
    ///   r#"<https://api.peopleforbikes.xyz/bnas?page=1>; rel="first", <https://api.peopleforbikes.xyz/bnas?page=2>; rel="prev", <https://api.peopleforbikes.xyz/bnas?page=4>; rel="next", <https://api.peopleforbikes.xyz/bnas?page=515>; rel="last""#
    /// );
    /// ```
    ///
    pub fn link_header(&self, defaul_page_size: Option<u64>) -> String {
        match &self.url {
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

#[derive(Serialize)]
pub struct PageFlow<T>
where
    T: Serialize,
{
    paginatron: Paginatron,
    payload: T,
}

impl<T> PageFlow<T>
where
    T: Serialize,
{
    pub fn new(paginatron: Paginatron, payload: T) -> Self {
        Self {
            paginatron,
            payload,
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

impl<T> From<PageFlow<T>> for Response<Body>
where
    T: Serialize,
{
    fn from(value: PageFlow<T>) -> Self {
        let nav = value.paginatron.navigation();
        Response::builder()
            .header(
                "link",
                value.paginatron.link_header(Some(DEFAULT_PAGE_SIZE)),
            )
            .header("x-next-page", nav.next())
            .header("x-page", value.paginatron.page)
            .header("x-per-page", value.paginatron.page_size)
            .header("x-prev-page", nav.prev())
            .header("x-total", value.paginatron.total_items)
            .header("x-total-pages", nav.last())
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_string(&value.payload).expect("payload must serialize"),
            ))
            .expect("failed to build response")
    }
}

impl<T> IntoResponse for PageFlow<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let nav = self.paginatron.navigation();
        let r: Response<axum::body::Body> = axum::response::Response::builder()
            .header("link", self.paginatron.link_header(Some(DEFAULT_PAGE_SIZE)))
            .header("x-next-page", nav.next())
            .header("x-page", self.paginatron.page)
            .header("x-per-page", self.paginatron.page_size)
            .header("x-prev-page", nav.prev())
            .header("x-total", self.paginatron.total_items)
            .header("x-total-pages", nav.last())
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&self.payload).expect("payload must serialize"),
            ))
            .expect("failed to build response");
        r
    }
}

/// Generates a TryFrom<&'a str> implementation for a specific type.
///
/// This macros expects the following items to be setup before use:
///   - the type must implement a `parse` function with the following signature
///     `pub fn parse(i: &str) -> IResult<&str, __type__>`
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
    match database_connect().await {
        Ok(db) => Ok(db),
        Err(e) => {
            let apigw_request_id = event.apigw_request_id();
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

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    ///  An error from unsuccessful database operations.
    #[error("database error: {0:?}")]
    DatabaseError(#[from] sea_orm::DbErr),

    /// Unexpected error.
    #[error("unexpected error: {0} {1}")]
    Unexpected(String, String),

    /// Entry not found.
    #[error("entry not found at {1}: {2}")]
    NotFound(Option<String>, String, String),

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
            ExecutionError::NotFound(id, source, message) => {
                APIError::not_found(id, &source, &message)
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
    fn into_response(self) -> axum::response::Response {
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

pub struct Context {
    // request_id: APIGatewayV2RequestID,
    request_id: Option<String>,
    source: String,
}

impl<S> FromRequest<S> for Context
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: axum::extract::Request, _: &S) -> Result<Self, Self::Rejection> {
        let request_context = req
            .extensions()
            .get::<lambda_http::request::RequestContext>();
        let request_id = match request_context {
            Some(lambda_http::request::RequestContext::ApiGatewayV2(ref ctx)) => {
                ctx.request_id.clone()
            }
            _ => None,
        };
        let uri = req.extensions().get::<OriginalUri>();
        let source = match uri {
            Some(path) => path.0.path().to_owned(),
            None => req.uri().path().to_owned(),
        };

        Ok(Self { request_id, source })
    }
}

impl Context {
    pub fn new(request_id: Option<String>, source: String) -> Self {
        Self { request_id, source }
    }

    pub fn request_id(&self) -> Option<String> {
        self.request_id.clone()
    }

    pub fn source(&self) -> String {
        self.source.to_owned()
    }
}

#[derive(Default, Deserialize)]
pub enum Sort {
    Asc,
    #[default]
    Desc,
}

derive_fromstr_from_deserialize!(Sort);

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lambda_events::http;
    use effortless::api::{parse_path_parameter, parse_request_body};
    use entity::wrappers::submission::SubmissionPost;
    use lambda_http::RequestExt;
    use std::collections::HashMap;

    #[test]
    fn test_paginatron_with_different_page_size() {
        let paginatron = Paginatron::new(
            Some("https://api.peopleforbikes.xyz/bnas".to_string()),
            200,
            3,
            25,
        );
        assert_eq!(
            paginatron.link_header(Some(50)),
            r#"<https://api.peopleforbikes.xyz/bnas?page_size=25&page=1>; rel="first", <https://api.peopleforbikes.xyz/bnas?page_size=25&page=2>; rel="prev", <https://api.peopleforbikes.xyz/bnas?page_size=25&page=4>; rel="next", <https://api.peopleforbikes.xyz/bnas?page_size=25&page=8>; rel="last""#
        );
    }

    #[test]
    fn test_paginatron_invalid_page() {
        let paginatron = Paginatron::new(
            Some("https://api.peopleforbikes.xyz/bnas".to_string()),
            42,
            3,
            25,
        );
        let nav = paginatron.navigation();
        assert_eq!(nav.first(), 1);
        assert_eq!(nav.prev(), 1);
        assert_eq!(nav.next(), 2);
        assert_eq!(nav.last(), 2);
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
