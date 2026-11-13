//! Request extractors answering rejected requests with JSON:API errors.
//!
//! They wrap the Axum `Path`, `Query`, and `Json` extractors, so that a request
//! with an invalid path parameter, query string, or body gets an `APIErrors`
//! document instead of the plain text message produced by Axum.
//!
//! `PageUrl` never rejects a request: it provides the URL the `link` header of
//! a list points at.
use crate::core::resource::schema::{APIError, APIErrorSource, APIErrors};
use axum::{
    body::{to_bytes, Body},
    extract::{FromRequest, FromRequestParts, OriginalUri, Path, Query, Request},
    http::{header, request::Parts, Extensions, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::convert::Infallible;

/// Largest request body, the default limit of the Axum extractors.
const BODY_LIMIT: usize = 2 * 1024 * 1024;

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
        // The database cannot store a NUL character.
        let path = request_path(&parts.uri, &parts.extensions);
        if path.contains("%00") {
            return Err(ExtractionError(APIError::new(
                request_id(&parts.extensions),
                StatusCode::BAD_REQUEST,
                "Invalid Path Parameter",
                "the path must not contain a NUL character",
                APIErrorSource::Pointer(path),
            )));
        }
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(Path(value)) => Ok(Self(value)),
            Err(rejection) => Err(ExtractionError(APIError::new(
                request_id(&parts.extensions),
                rejection.status(),
                "Invalid Path Parameter",
                &rejection.body_text(),
                APIErrorSource::Pointer(request_path(&parts.uri, &parts.extensions)),
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
                    None => APIErrorSource::Pointer(request_path(&parts.uri, &parts.extensions)),
                };
                Err(ExtractionError(APIError::new(
                    request_id(&parts.extensions),
                    rejection.status(),
                    "Invalid Query String Parameter",
                    &details,
                    source,
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
        let path = request_path(req.uri(), req.extensions());
        let (parts, body) = req.into_parts();
        let bytes = match to_bytes(body, BODY_LIMIT).await {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(ExtractionError(APIError::new(
                    id,
                    StatusCode::BAD_REQUEST,
                    "Invalid Request Body",
                    &e.to_string(),
                    APIErrorSource::Pointer(path),
                )))
            }
        };
        let req = Request::from_parts(parts, Body::from(bytes.clone()));
        match Json::<T>::from_request(req, state).await {
            // serde also reads a struct from an array, matching its fields by
            // position, but every body is documented as an object.
            Ok(_) if !is_object(&bytes) => Err(ExtractionError(APIError::invalid_attribute(
                id,
                &path,
                "the body must be a JSON object",
            ))),
            // The database cannot store a NUL character.
            Ok(_) if holds_nul(&bytes) => Err(ExtractionError(APIError::invalid_attribute(
                id,
                &path,
                "a string of the body must not contain a NUL character",
            ))),
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) => {
                let details = rejection.body_text();
                // A missing content type is a 415 and valid JSON with rejected
                // content is a 422. Any other body problem is a bad request.
                let status = match rejection.status() {
                    status @ (StatusCode::UNSUPPORTED_MEDIA_TYPE
                    | StatusCode::UNPROCESSABLE_ENTITY) => status,
                    _ => StatusCode::BAD_REQUEST,
                };
                let source = if status == StatusCode::UNSUPPORTED_MEDIA_TYPE {
                    APIErrorSource::Header(header::CONTENT_TYPE.to_string())
                } else {
                    APIErrorSource::Pointer(path)
                };
                Err(ExtractionError(APIError::new(
                    id,
                    status,
                    "Invalid Request Body",
                    &details,
                    source,
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

/// Returns whether a valid JSON body is an object.
fn is_object(body: &[u8]) -> bool {
    body.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'{')
}

/// Returns whether a valid JSON body holds a NUL character in a string or a key.
fn holds_nul(body: &[u8]) -> bool {
    fn walk(value: &Value) -> bool {
        match value {
            Value::String(s) => s.contains('\0'),
            Value::Array(items) => items.iter().any(walk),
            Value::Object(fields) => fields
                .iter()
                .any(|(key, value)| key.contains('\0') || walk(value)),
            _ => false,
        }
    }
    // Only an escaped NUL is valid JSON, so most bodies skip the second parsing.
    body.windows(6).any(|w| w == br"\u0000")
        && serde_json::from_slice::<Value>(body).is_ok_and(|value| walk(&value))
}

/// Returns the API Gateway request ID, when the request came through it.
fn request_id(extensions: &Extensions) -> Option<String> {
    match extensions.get::<lambda_http::request::RequestContext>() {
        Some(lambda_http::request::RequestContext::ApiGatewayV2(ctx)) => ctx.request_id.clone(),
        _ => None,
    }
}

/// Returns the path of the request as sent by the client.
fn request_path(uri: &Uri, extensions: &Extensions) -> String {
    match extensions.get::<OriginalUri>() {
        Some(original) => original.0.path().to_owned(),
        None => uri.path().to_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_object() {
        assert!(is_object(br#"{"a": 1}"#));
        assert!(is_object(b" \n\t{}"));
        assert!(!is_object(b"[null, null]"));
        assert!(!is_object(b" []"));
        assert!(!is_object(b"null"));
        assert!(!is_object(br#""text""#));
        assert!(!is_object(b"42"));
    }

    #[test]
    fn test_holds_nul() {
        assert!(holds_nul(br#"{"a": "x\u0000y"}"#));
        assert!(holds_nul(br#"{"a": ["\u0000"]}"#));
        assert!(holds_nul(br#"{"a\u0000": 1}"#));
        assert!(!holds_nul(br#"{"a": "x"}"#));
        // An escaped backslash followed by the text u0000 is not a NUL.
        assert!(!holds_nul(br#"{"a": "\\u0000"}"#));
    }

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
    fn test_error_path_unknown_field() {
        assert_eq!(
            error_path("Failed to deserialize query string: latest: unknown field `latest`, expected one of `page`, `page_size`"),
            Some(String::from("latest"))
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
}
