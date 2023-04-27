pub mod link_header;

use lambda_http::{Body, Error, Request, RequestExt, Response};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, env, num::ParseIntError};

/// Maximum number of items allowed to be returned by a query at once.
pub const MAX_PAGE_SIZE: u64 = 100;
/// Number of items to return per page if no argument was provided.
pub const DEFAULT_PAGE_SIZE: u64 = 50;

/// Represent the contents of the encrypted fields SecretString or SecretBinary
/// from the specified version of a secret, whichever contains content.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretValue {
    /// Amazon Resource Name of the secret.
    #[serde(alias = "ARN")]
    pub arn: String,
    /// Creation date.
    pub created_date: String,
    /// The friendly name of the secret.
    pub name: String,
    /// The decrypted secret value, if the secret value was originally provided
    /// as binary data in the form of a byte array. The response parameter
    /// represents the binary data as a base64-encoded string.
    ///
    /// If the secret was created by using the Secrets Manager console, or if
    /// the secret value was originally provided as a string, then this field
    /// is omitted. The secret value appears in SecretString instead.
    pub secret_binary: Option<String>,
    /// The decrypted secret value, if the secret value was originally provided
    /// as a string or through the Secrets Manager console.
    /// If this secret was created by using the console, then Secrets Manager
    /// stores the information as a JSON structure of key/value pairs.
    pub secret_string: String,
    /// Unique identifier of the version of the secret.
    pub version_id: String,
    /// A list of all of the staging labels currently attached to this version
    /// of the secret.
    pub version_stages: Vec<String>,
    /// Metadata.
    pub result_metadata: HashMap<String, String>,
}

/// Return the database connection.
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
                  Ok(v) => {
                    let secret_value = get_aws_secrets(&v)
                        .await
                        .map_err( DbErr::Custom)?;
                    let secrets: HashMap<String, String> = serde_json::from_str(&secret_value.secret_string)
                        .map_err(|e| DbErr::Custom(format!("Cannot deserialize the cached secret: {e}")))?;
                    match secrets.get(DATABASE_URL_KEY) {
                      Some(v) => v.to_owned(),
                      None => return Err(DbErr::Custom(format!("Cannot find the connection string within the secret {:?}. Ensure `{DATABASE_URL_KEY}` is correctly set.", secret_id))),
                    }
                  },
                  Err(e) => return Err(DbErr::Custom(format!("Cannot find the connection string in the AWS Secrets Manager. Ensure `{:?}` is correctly set. Reason: {e}", secret_id))),
            }
            None => return Err(DbErr::Custom(format!("Cannot find the connection string. Ensure `{DATABASE_URL_KEY}` is correctly set."))),
        },
    };
    Database::connect(database_url).await
}

/// Retrieve a secret from the AWS Secrets Manager using the Lambda caching layer.
///
/// Ref: https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieving-secrets_lambda.html
pub async fn get_aws_secrets(secret_id: &str) -> Result<SecretValue, String> {
    let aws_session_token =
        env::var("AWS_SESSION_TOKEN").map_err(|e| format!("Cannot find AWS session token: {e}"))?;
    reqwest::Client::new()
        .get(format!(
            "http://localhost:2773/secretsmanager/get?secretId={secret_id}"
        ))
        .header("X-Aws-Parameters-Secrets-Token", aws_session_token)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<SecretValue>()
        .await
        .map_err(|e| e.to_string())
}

/// Retrieve the pagination parameters.
///
/// If nothing is provided, the first page is returned and will contain up to
/// `[DEFAULT_PAGE_SIZE]` items.
///
/// If `page` does not exist, an empty array is returned.
pub fn pagination_parameters(event: &Request) -> Result<(u64, u64), ParseIntError> {
    let mut page_size = event
        .query_string_parameters()
        .first("page_size")
        .unwrap_or(DEFAULT_PAGE_SIZE.to_string().as_str())
        .parse::<u64>()?;
    let page = event
        .query_string_parameters()
        .first("page")
        .unwrap_or("1")
        .parse::<u64>()?;

    // Ensure we do not allow the page to go above the max size.
    if page_size > MAX_PAGE_SIZE {
        page_size = MAX_PAGE_SIZE;
    }
    Ok((page_size, page))
}

/// Build a paginated Response.
///
/// Builds a Response struct which contains the pagination information in the headers.
///
/// Implements pagination headers, similar to GitLab:
/// https://docs.gitlab.com/ee/api/rest/index.html#other-pagination-headers
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
    // Prepare the pagination values.
    let total_pages = (total_items + page_size - 1) / page_size;
    let previous_page = if page <= 1 { 1 } else { page - 1 };
    let next_page = if page >= total_pages {
        total_pages
    } else {
        page + 1
    };

    // Retrieve the URL if present.
    let uri = event.uri();
    let mut link_header: String = String::new();
    if uri.host().is_some() {
        let uri_scheme = uri.scheme_str().unwrap_or("https");
        let uri_path = uri.path();
        let mut url = format!("{}://{}/{}", uri_scheme, uri.host().unwrap(), uri_path);
        if page_size != DEFAULT_PAGE_SIZE {
            url.push_str("page_size={page_size}");
        }

        // Build the link header.
        link_header = assemble_link_header_string(&url, previous_page, next_page, total_pages);
    }

    // Build the response.
    Ok(Response::builder()
        .header("link", link_header)
        .header("x-next-page", next_page)
        .header("x-page", page)
        .header("x-per-page", page_size)
        .header("x-prev-page", previous_page)
        .header("x-total", total_items)
        .header("x-total-pages", total_pages)
        .body(body.to_string().into())
        .map_err(Box::new)?)
}

/// Concatenates the different segments of the link header.
fn assemble_link_header_string(url: &str, prev: u64, next: u64, last: u64) -> String {
    let first = format!(r#"<{url}?page=1>; rel="first""#);
    let prev = format!(r#"<{url}?page={prev}>; rel="prev""#);
    let next = format!(r#"<{url}?page={next}>; rel="next""#);
    let last = format!(r#"<{url}?page={last}>; rel="last""#);
    format!("{first}, {prev}, {next}, {last}")
}

// Create a TryFrom<&'a str> implementation for a specific type.
//
// This macros expects the following items to be setup before use:
//   - the type must implement a `parse` function with the following signature
//       `pub fn parse(i: &str) -> IResult<&str, __type__>`
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
