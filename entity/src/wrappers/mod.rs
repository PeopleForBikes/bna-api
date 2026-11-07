pub mod bna_pipeline;
pub mod city;
pub mod submission;

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Approved,
    Pending,
    Rejected,
}

impl FromStr for ApprovalStatus {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for ApprovalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BNAPipelineStatus {
    Pending,
    Processing,
    Completed,
}

impl FromStr for BNAPipelineStatus {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for BNAPipelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

/// Pipeline steps, as documented by the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BNAPipelineStep {
    Setup,
    Analysis,
    Save,
    Cleanup,
}

impl FromStr for BNAPipelineStep {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for BNAPipelineStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

/// BNA regions of the United States, as stored in the `bna_region` table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum BnaRegion {
    #[serde(rename = "Mid-Atlantic")]
    MidAtlantic,
    #[serde(rename = "New England")]
    NewEngland,
    Midwest,
    Mountain,
    Pacific,
    South,
}

impl FromStr for BnaRegion {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for BnaRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

/// Bike lane types, as stored in the `bike_lane_type` table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BikeLaneType {
    BufferedLane,
    Lane,
    Path,
    Sharrow,
    Track,
}

impl FromStr for BikeLaneType {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for BikeLaneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The names of the regions, as filled in the `bna_region` table.
    #[test]
    fn test_bnaregion_matches_the_table() {
        for (region, name) in [
            (BnaRegion::MidAtlantic, "Mid-Atlantic"),
            (BnaRegion::Midwest, "Midwest"),
            (BnaRegion::Mountain, "Mountain"),
            (BnaRegion::NewEngland, "New England"),
            (BnaRegion::Pacific, "Pacific"),
            (BnaRegion::South, "South"),
        ] {
            assert_eq!(region.to_string(), name);
            assert_eq!(name.parse::<BnaRegion>().unwrap(), region);
        }
    }

    // The names of the bike lane types, as filled in the `bike_lane_type` table.
    #[test]
    fn test_bikelanetype_matches_the_table() {
        for (lane, name) in [
            (BikeLaneType::BufferedLane, "buffered_lane"),
            (BikeLaneType::Lane, "lane"),
            (BikeLaneType::Path, "path"),
            (BikeLaneType::Sharrow, "sharrow"),
            (BikeLaneType::Track, "track"),
        ] {
            assert_eq!(lane.to_string(), name);
            assert_eq!(name.parse::<BikeLaneType>().unwrap(), lane);
        }
    }
}
