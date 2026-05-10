use bon::Builder;
use lambda_http::{http::header, http::StatusCode, Body, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorSource {
    Pointer(String),
    Parameter(String),
    Header(String),
}

/// Single API Error object as described in <https://jsonapi.org/format/#error-objects>.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Builder)]
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

    /// Creates a new `APIError` based off a query parameter error.
    pub fn with_parameter(id: Option<String>, parameter: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::BAD_REQUEST.as_u16().to_string(),
            title: String::from("Invalid Query Sring Parameter"),
            source: Some(APIErrorSource::Parameter(parameter.into())),
            details: message.into(),
        }
    }

    /// Creates a new `APIError` based off an invalid attribute.
    pub fn with_pointer(id: Option<String>, pointer: &str, message: &str) -> Self {
        Self {
            id,
            status: StatusCode::BAD_REQUEST.as_u16().to_string(),
            title: String::from("Invalid Attribute"),
            source: Some(APIErrorSource::Pointer(pointer.into())),
            details: message.into(),
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

    /// Returns the APIError status.
    pub fn status(&self) -> String {
        self.status.clone()
    }
}

impl From<APIError> for Response<Body> {
    fn from(value: APIError) -> Self {
        let errors: APIErrors = value.into();
        errors.into()
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

    /// Creates an empty `APIErrors`.
    pub fn empty() -> Self {
        Self { errors: vec![] }
    }

    /// Adds an `APIError`.
    pub fn add(mut self, value: APIError) {
        self.errors.push(value);
    }

    /// Extends with an existing `APIErrors`.
    pub fn extend(&mut self, value: APIErrors) {
        self.errors.extend(value.errors);
    }

    /// Returns True if there is no error.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the errors field.
    pub fn errors(&self) -> &[APIError] {
        self.errors.as_slice()
    }
}

impl From<APIError> for APIErrors {
    fn from(value: APIError) -> Self {
        APIErrors {
            errors: vec![value],
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
            value.errors.first().unwrap().status.as_bytes()
        } else {
            StatusCode::BAD_REQUEST.as_str().as_bytes()
        };
        Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!(value).to_string().into())
            .unwrap()
    }
}
