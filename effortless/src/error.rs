use lambda_http::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object containing references to the primary source of the error.
///
/// It SHOULD include one of the following members or be omitted:
///
///   - pointer: the path of the request that caused the error [e.g. "/ratings/1"]. The
///     error details name the invalid field, if any.
///   - parameter: a string indicating which URI query parameter caused the error.
///   - header: a string indicating the name of a single request header which caused the
///     error.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorSource {
    Pointer(String),
    Parameter(String),
    Header(String),
}

/// Single API Error object as described in <https://jsonapi.org/format/#error-objects>.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct APIError {
    /// A unique identifier for this particular occurrence of the problem.
    id: Option<String>,
    /// The HTTP status code applicable to this problem, expressed as a string value.
    status: String,
    /// A short, human-readable summary of the problem
    title: String,
    /// A human-readable explanation specific to this occurrence of the problem
    details: String,
    /// An object containing references to the primary source of the error.
    ///
    /// This field may be omitted in some situation. For instance, if the server cannot
    /// parse the request as valid JSON, including source doesn’t make sense
    /// (because there’s no JSON document for source to refer to).
    source: Option<APIErrorSource>,
}

impl APIError {
    /// Creates a new `APIError`.
    pub fn new(
        id: Option<String>,
        status: StatusCode,
        title: &str,
        details: &str,
        source: Option<APIErrorSource>,
    ) -> Self {
        Self {
            id,
            status: status.as_u16().to_string(),
            title: title.to_string(),
            details: details.to_string(),
            source,
        }
    }

    // Creates a new `APIError` for internal errors.
    pub fn internal_error(id: Option<String>, title: &str, details: &str, source: &str) -> Self {
        Self {
            id,
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16().to_string(),
            source: Some(APIErrorSource::Pointer(source.into())),
            title: title.into(),
            details: details.into(),
        }
    }

    /// Creates a new `APIError` for database issues.
    pub fn db_error(id: Option<String>, source: &str, message: &str) -> Self {
        APIError::internal_error(id, "Database error", message, source)
    }

    /// Creates a new `APIError` for no-content issues.
    pub fn not_found(id: Option<String>, source: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::NOT_FOUND.as_u16().to_string(),
            title: String::from("Content Not Found"),
            source: Some(APIErrorSource::Pointer(source.into())),
            details: message.into(),
        }
    }

    /// Creates a new `APIError` for a request body whose content is rejected.
    pub fn invalid_attribute(id: Option<String>, source: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::UNPROCESSABLE_ENTITY.as_u16().to_string(),
            title: String::from("Invalid Attribute"),
            source: Some(APIErrorSource::Pointer(source.into())),
            details: message.into(),
        }
    }

    /// Creates a new `APIError` for a resource that already exists.
    pub fn conflict(id: Option<String>, source: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::CONFLICT.as_u16().to_string(),
            title: String::from("Conflict"),
            source: Some(APIErrorSource::Pointer(source.into())),
            details: message.into(),
        }
    }

    /// Returns the APIError status.
    pub fn status(&self) -> String {
        self.status.clone()
    }
}

/// Error objects MUST be returned as an array keyed by errors in the top level of a
/// JSON:API document.
#[derive(Deserialize, Serialize, Debug)]
pub struct APIErrors {
    pub errors: Vec<APIError>,
}

impl APIErrors {
    /// Creates a new `APIErrors`.
    pub fn new(errors: &[APIError]) -> Self {
        Self {
            errors: errors.to_vec(),
        }
    }
}

impl From<APIError> for APIErrors {
    fn from(value: APIError) -> Self {
        APIErrors {
            errors: vec![value],
        }
    }
}
