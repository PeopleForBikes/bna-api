use crate::Sort;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use query_map::QueryMap;

pub mod adaptor;
mod db;
pub mod endpoint;

pub struct PriceParameters {
    sort: Option<Sort>,
    latest: bool,
}

impl PriceParameters {
    pub fn new(sort: Option<Sort>, latest: bool) -> Self {
        Self { sort, latest }
    }

    pub fn set_sort(&mut self, sort: Option<Sort>) {
        self.sort = sort;
    }

    pub fn set_latest(&mut self, latest: bool) {
        self.latest = latest;
    }
}

impl Default for PriceParameters {
    fn default() -> Self {
        Self {
            sort: Some(Sort::Desc),
            latest: false,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for PriceParameters
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let uri = &parts.uri;
        if let Some(q) = uri.query() {
            let map = q.parse::<QueryMap>().unwrap();
            let sort = match map.first("sort") {
                Some(s) => match serde_json::from_str::<Sort>(s) {
                    Ok(v) => Some(v),
                    Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
                },
                None => Some(Sort::default()),
            };
            let latest = match map.first("latest") {
                Some(s) => match serde_json::from_str::<bool>(s) {
                    Ok(v) => v,
                    Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
                },
                None => false,
            };

            Ok(PriceParameters { sort, latest })
        } else {
            Ok(PriceParameters::default())
        }
    }
}
