pub mod bna;
pub mod bna_pipeline;
pub mod city;
pub mod submission;

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BNAPipelineStep {
    Analysis,
    Cleanup,
    Setup,
    Save,
}

impl FromStr for BNAPipelineStep {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bnaregion_ser() {
        assert_eq!(
            BnaRegion::MidAtlantic.to_string(),
            String::from("Mid-Atlantic")
        );
        assert_eq!(
            BnaRegion::NewEngland.to_string(),
            String::from("New England")
        );
        assert_eq!(BnaRegion::South.to_string(), String::from("South"));
    }
}
