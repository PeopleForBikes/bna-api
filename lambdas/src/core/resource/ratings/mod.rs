use effortless::{
    api::{parse_path_parameter, parse_query_string_parameter},
    error::APIErrors,
};
use lambda_http::Request;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub mod adaptor;
mod db;
pub mod endpoint;
mod schema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BNAComponent {
    All,
    Summary,
    Infratructure,
    Recreation,
    Opportunity,
    CoreServices,
    People,
    Retail,
    Transit,
}

impl FromStr for BNAComponent {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

/// Path parameters for the /ratings enpoint.
pub struct BNAPathParameters {
    pub rating_id: Uuid,
}

/// Query parameters for the /bnas enpoint.
pub struct BNAQueryParameters {
    pub component: BNAComponent,
}

pub fn extract_path_parameters(event: &Request) -> Result<BNAPathParameters, APIErrors> {
    let rating_id =
        parse_path_parameter::<Uuid>(event, "rating_id")?.expect("no rating_id parameter provided");

    Ok(BNAPathParameters { rating_id })
}

pub fn extract_query_parameters(event: &Request) -> Result<BNAQueryParameters, APIErrors> {
    let component = parse_query_string_parameter::<BNAComponent>(event, "component")?
        .unwrap_or(BNAComponent::All);

    Ok(BNAQueryParameters { component })
}
