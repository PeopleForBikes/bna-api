//! Request extractors answering rejected requests with JSON:API errors.
//!
//! They wrap the Axum `Path`, `Query`, and `Json` extractors, so that a request
//! with an invalid path parameter, query string, or body gets an `APIErrors`
//! document instead of the plain text message produced by Axum.
//!
//! `PageUrl` never rejects a request: it provides the URL the `link` header of
//! a list points at.
use axum::{
    extract::{FromRequest, FromRequestParts, OriginalUri, Path, Query, Request},
    http::{header, request::Parts, Extensions, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use effortless::error::{APIError, APIErrorSource, APIErrors};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::convert::Infallible;

/// Path parameters, rejected with an `APIErrors` document.
pub struct ApiPath<T>(pub T);

/// Query string parameters, rejected with an `APIErrors` document.
pub struct ApiQuery<T>(pub T);

/// JSON request body, rejected with an `APIErrors` document.
pub struct ApiJson<T>(pub T);

/// URL of a list request, without the pagination query parameters.
///
/// The other query parameters, such as filters and sorting, are kept, so that
/// the `link` header navigates the same list. It is `None` when the request
/// does not say which host it was sent to.
pub struct PageUrl(pub Option<String>);

/// A rejected request, rendered as an `APIErrors` document.
#[derive(Debug)]
pub struct ExtractionError(APIError);

impl IntoResponse for ExtractionError {
    fn into_response(self) -> Response {
        let status = self
            .0
            .status()
            .parse::<u16>()
            .ok()
            .and_then(|s| StatusCode::from_u16(s).ok())
            .unwrap_or(StatusCode::BAD_REQUEST);
        let errors = APIErrors::new(&[self.0]);
        (
            status,
            [(header::CONTENT_TYPE, "application/json")],
            json!(errors).to_string(),
        )
            .into_response()
    }
}

impl<T, S> FromRequestParts<S> for ApiPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ExtractionError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(Path(value)) => Ok(Self(value)),
            Err(rejection) => Err(ExtractionError(APIError::new(
                request_id(&parts.extensions),
                rejection.status(),
                "Invalid Path Parameter",
                &rejection.body_text(),
                Some(APIErrorSource::Pointer(request_path(parts))),
            ))),
        }
    }
}

impl<T, S> FromRequestParts<S> for ApiQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ExtractionError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Query::<T>::from_request_parts(parts, state).await {
            Ok(Query(value)) => Ok(Self(value)),
            Err(rejection) => {
                let details = rejection.body_text();
                let source = match error_path(&details) {
                    Some(parameter) => APIErrorSource::Parameter(parameter),
                    None => APIErrorSource::Pointer(request_path(parts)),
                };
                Err(ExtractionError(APIError::new(
                    request_id(&parts.extensions),
                    rejection.status(),
                    "Invalid Query String Parameter",
                    &details,
                    Some(source),
                )))
            }
        }
    }
}

impl<T, S> FromRequest<S> for ApiJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ExtractionError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let id = request_id(req.extensions());
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) => {
                let details = rejection.body_text();
                let source = if rejection.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE {
                    APIErrorSource::Header(header::CONTENT_TYPE.to_string())
                } else {
                    APIErrorSource::Pointer(body_pointer(&details))
                };
                // Every body problem is a bad request, including the 415 and 422
                // statuses Axum uses for a missing content type or invalid data.
                Err(ExtractionError(APIError::new(
                    id,
                    StatusCode::BAD_REQUEST,
                    "Invalid Request Body",
                    &details,
                    Some(source),
                )))
            }
        }
    }
}

impl<S> FromRequestParts<S> for PageUrl
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let uri = match parts.extensions.get::<OriginalUri>() {
            Some(uri) => &uri.0,
            None => &parts.uri,
        };
        // Lambda requests carry an absolute URI; local requests only carry a
        // path, and name their host in the headers.
        let header = |name| {
            parts
                .headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        let Some(authority) = uri
            .authority()
            .map(|a| a.to_string())
            .or_else(|| header(header::HOST))
        else {
            return Ok(Self(None));
        };
        let scheme = uri
            .scheme_str()
            .map(str::to_owned)
            .or_else(|| header(header::HeaderName::from_static("x-forwarded-proto")))
            .unwrap_or_else(|| String::from("http"));
        Ok(Self(Some(page_url(
            &scheme,
            &authority,
            uri.path(),
            uri.query(),
        ))))
    }
}

/// Builds the URL of a list from the parts of its request URL, dropping the
/// `page` and `page_size` query parameters.
fn page_url(scheme: &str, authority: &str, path: &str, query: Option<&str>) -> String {
    let kept = query
        .unwrap_or_default()
        .split('&')
        .filter(|pair| {
            let key = pair.split_once('=').map_or(*pair, |(key, _)| key);
            !pair.is_empty() && key != "page" && key != "page_size"
        })
        .collect::<Vec<_>>()
        .join("&");
    if kept.is_empty() {
        format!("{scheme}://{authority}{path}")
    } else {
        format!("{scheme}://{authority}{path}?{kept}")
    }
}

/// Returns the API Gateway request ID, when the request came through it.
fn request_id(extensions: &Extensions) -> Option<String> {
    match extensions.get::<lambda_http::request::RequestContext>() {
        Some(lambda_http::request::RequestContext::ApiGatewayV2(ctx)) => ctx.request_id.clone(),
        _ => None,
    }
}

/// Returns the path of the request as sent by the client.
fn request_path(parts: &Parts) -> String {
    match parts.extensions.get::<OriginalUri>() {
        Some(uri) => uri.0.path().to_owned(),
        None => parts.uri.path().to_owned(),
    }
}

/// Extracts the path of the failing field from an Axum deserialization message.
///
/// Axum reports errors as `<context>: <path>: <reason>`, where `<path>` is
/// omitted for errors at the top level, such as a missing field. In that case
/// the name of the missing field is returned.
fn error_path(details: &str) -> Option<String> {
    let reason = details.split_once(": ").map(|(_, rest)| rest)?;
    if let Some(field) = reason
        .strip_prefix("missing field `")
        .and_then(|rest| rest.split_once('`'))
        .map(|(field, _)| field)
    {
        return Some(field.to_string());
    }
    let (path, _) = reason.split_once(": ")?;
    let is_path = !path.is_empty()
        && path
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '[' | ']'));
    is_path.then(|| path.to_string())
}

/// Converts the failing field of a JSON body error into a JSON pointer.
///
/// `core_services.score` becomes `/core_services/score` and `items[0]` becomes
/// `/items/0`. Errors without a field point at the whole document (`/`).
fn body_pointer(details: &str) -> String {
    match error_path(details) {
        Some(path) => {
            let segments = path
                .replace('[', ".")
                .replace(']', "")
                .split('.')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("/");
            format!("/{segments}")
        }
        None => String::from("/"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_url_without_query() {
        assert_eq!(
            page_url("https", "api.example.org", "/ratings", None),
            "https://api.example.org/ratings"
        );
    }

    #[test]
    fn test_page_url_keeps_other_parameters() {
        assert_eq!(
            page_url(
                "http",
                "localhost:3000",
                "/cities/submissions",
                Some("page=2&status=Pending&page_size=1&sort_by=name")
            ),
            "http://localhost:3000/cities/submissions?status=Pending&sort_by=name"
        );
    }

    #[test]
    fn test_page_url_only_pagination_parameters() {
        assert_eq!(
            page_url(
                "http",
                "localhost:3000",
                "/usstates",
                Some("page=2&page_size=1")
            ),
            "http://localhost:3000/usstates"
        );
    }

    #[test]
    fn test_error_path_query_parameter() {
        assert_eq!(
            error_path("Failed to deserialize query string: page: invalid digit found in string"),
            Some(String::from("page"))
        );
    }

    #[test]
    fn test_error_path_missing_field() {
        assert_eq!(
            error_path("Failed to deserialize the JSON body into the target type: missing field `country` at line 1 column 12"),
            Some(String::from("country"))
        );
    }

    #[test]
    fn test_error_path_without_field() {
        assert_eq!(
            error_path(
                "Failed to parse the request body as JSON: expected ident at line 1 column 2"
            ),
            None
        );
    }

    #[test]
    fn test_body_pointer_nested() {
        assert_eq!(
            body_pointer("Failed to deserialize the JSON body into the target type: core_services.score: invalid type: string \"a\", expected f64 at line 1 column 40"),
            "/core_services/score"
        );
    }

    #[test]
    fn test_body_pointer_index() {
        assert_eq!(
            body_pointer(
                "Failed to deserialize the JSON body into the target type: items[0]: invalid type"
            ),
            "/items/0"
        );
    }

    #[test]
    fn test_body_pointer_root() {
        assert_eq!(
            body_pointer(
                "Failed to parse the request body as JSON: expected ident at line 1 column 2"
            ),
            "/"
        );
    }
}
