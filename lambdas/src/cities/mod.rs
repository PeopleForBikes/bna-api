//! Module for the /cities enpoint.
use axum::{
    async_trait,
    extract::{FromRequest, OriginalUri, Request},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use effortless::{
    api::parse_path_parameter,
    error::{APIError, APIErrors},
};
use serde::Deserialize;
use serde_json::json;
use thiserror;

pub mod adaptor;
mod db;
pub mod endpoint;

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    ///  An error from unsuccessful database operations.
    #[error("database error")]
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
    fn into_response(self) -> Response {
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

/// Path parameters for the /cities enpoint.
#[derive(Debug, Deserialize)]
pub struct CitiesPathParameters {
    /// Country name.
    pub country: String,
    /// Region name.
    pub region: String,
    /// City name.
    pub name: String,
}

/// Extract the path parameters for the /cities endpoint.
pub fn extract_path_parameters(
    event: &lambda_http::Request,
) -> Result<CitiesPathParameters, APIErrors> {
    let mut api_errors = APIErrors::empty();

    let country = match parse_path_parameter::<String>(event, "country") {
        Ok(value) => value,
        Err(e) => {
            api_errors.extend(e);
            None
        }
    };

    let region = match parse_path_parameter::<String>(event, "region") {
        Ok(value) => value,
        Err(e) => {
            api_errors.extend(e);
            None
        }
    };
    let name = match parse_path_parameter::<String>(event, "name") {
        Ok(value) => value,
        Err(e) => {
            api_errors.extend(e);
            None
        }
    };

    if !api_errors.is_empty() {
        return Err(api_errors);
    }

    Ok(CitiesPathParameters {
        country: country.unwrap(),
        region: region.unwrap(),
        name: name.unwrap(),
    })
}

// pub struct APIGatewayV2RequestID(String);

// impl APIGatewayV2RequestID {
//     pub fn request_id(&self) -> String {
//         self.0.to_owned()
//     }
// }

// #[async_trait]
// impl<S> FromRequest<S> for APIGatewayV2RequestID
// where
//     Bytes: FromRequest<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, &'static str);

//     #[doc = " Perform the extraction."]
//     #[must_use]
//     #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
//         let context = req
//             .extensions()
//             .get::<lambda_http::request::RequestContext>();
//         if let Some(lambda_http::request::RequestContext::ApiGatewayV2(ctx)) = context {
//             Ok(APIGatewayV2RequestID(
//                 ctx.request_id
//                     .clone()
//                     .expect("a request coming from API Gateway V2"),
//             ))
//         } else {
//             Err((StatusCode::BAD_REQUEST, "API Gateway Request ID is missing"))
//         }
//     }
// }

pub struct Context {
    // request_id: APIGatewayV2RequestID,
    request_id: Option<String>,
    source: String,
}

#[async_trait]
impl<S> FromRequest<S> for Context
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        let request_context = req
            .extensions()
            .get::<lambda_http::request::RequestContext>();
        let request_id = match request_context {
            Some(&lambda_http::request::RequestContext::ApiGatewayV2(ref ctx)) => {
                ctx.request_id.clone()
            }
            _ => None,
        };

        // .and_then(|c| match c {
        //     lambda_http::request::RequestContext::ApiGatewayV2(context) => context.request_id,
        //     _ => None,
        // });
        let uri = req.extensions().get::<OriginalUri>();
        let source = match uri {
            Some(path) => path.0.path().to_owned(),
            None => req.uri().path().to_owned(),
        };
        // let context = req
        //     .extensions()
        //     .get::<lambda_http::request::RequestContext>();
        // let request_id = match context {
        //     Some(lambda_http::request::RequestContext::ApiGatewayV2(ctx)) => ctx
        //         .request_id
        //         .clone()
        //         .expect("a request coming from API Gateway V2"),
        //     _ => return Err((StatusCode::BAD_REQUEST, "API Gateway Request ID is missing")),
        // };
        // let uri = req.extensions().get::<OriginalUri>();
        // let source = match uri {
        //     Some(path) => path.0.path().to_owned(),
        //     None => req.uri().path().to_owned(),
        // };
        Ok(Self { request_id, source })
    }
}

// #[async_trait]
// impl<S> FromRequest<S> for Context
// where
//     Bytes: FromRequest<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, &'static str);

//     #[doc = " Perform the extraction."]
//     #[must_use]
//     #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let context = req
//             .extensions()
//             .get::<lambda_http::request::RequestContext>();
//         let request_id = match context {
//             Some(lambda_http::request::RequestContext::ApiGatewayV2(ctx)) => ctx
//                 .request_id
//                 .clone()
//                 .expect("a request coming from API Gateway V2"),
//             _ => return Err((StatusCode::BAD_REQUEST, "API Gateway Request ID is missing")),
//         };
//         let uri = req.extensions().get::<OriginalUri>();
//         let source = match uri {
//             Some(path) => path.0.path().to_owned(),
//             None => req.uri().path().to_owned(),
//         };
//         Ok(Context {
//             request_id: APIGatewayV2RequestID(request_id),
//             source,
//         })
//     }
// }

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

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{http, Body, RequestExt};
    use std::collections::HashMap;

    #[test]
    fn test_extract_path_parameters() {
        let country: String = String::from("United States");
        let region: String = String::from("Texas");
        let name: String = String::from("Austin");
        let event = http::Request::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::Empty)
            .expect("failed to build request")
            .with_path_parameters(HashMap::from([
                ("country".to_string(), country.clone()),
                ("region".to_string(), region.clone()),
                ("name".to_string(), name.clone()),
            ]))
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let r = extract_path_parameters(&event).unwrap();
        assert_eq!(r.country, country);
        assert_eq!(r.region, region);
        assert_eq!(r.name, name);
    }
}
