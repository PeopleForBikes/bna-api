pub mod bna;
pub mod brokenspoke_pipeline;
pub mod census;
pub mod city;
pub mod submission;

use crate::entities::sea_orm_active_enums;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Approved,
    Pending,
    Rejected,
}

impl From<sea_orm_active_enums::ApprovalStatus> for ApprovalStatus {
    fn from(value: sea_orm_active_enums::ApprovalStatus) -> Self {
        match value {
            sea_orm_active_enums::ApprovalStatus::Approved => Self::Approved,
            sea_orm_active_enums::ApprovalStatus::Rejected => Self::Rejected,
            sea_orm_active_enums::ApprovalStatus::Pending => Self::Pending,
        }
    }
}

impl From<ApprovalStatus> for sea_orm_active_enums::ApprovalStatus {
    fn from(val: ApprovalStatus) -> Self {
        match val {
            ApprovalStatus::Approved => sea_orm_active_enums::ApprovalStatus::Approved,
            ApprovalStatus::Rejected => sea_orm_active_enums::ApprovalStatus::Rejected,
            ApprovalStatus::Pending => sea_orm_active_enums::ApprovalStatus::Pending,
        }
    }
}

impl FromStr for ApprovalStatus {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrokenspokeStep {
    SqsMessage,
    Setup,
    Analysis,
    Cleanup,
}

impl From<sea_orm_active_enums::BrokenspokeStep> for BrokenspokeStep {
    fn from(value: sea_orm_active_enums::BrokenspokeStep) -> Self {
        match value {
            sea_orm_active_enums::BrokenspokeStep::Analysis => Self::Analysis,
            sea_orm_active_enums::BrokenspokeStep::Cleanup => Self::Cleanup,
            sea_orm_active_enums::BrokenspokeStep::Setup => Self::Setup,
            sea_orm_active_enums::BrokenspokeStep::SqsMessage => Self::SqsMessage,
        }
    }
}

impl From<BrokenspokeStep> for sea_orm_active_enums::BrokenspokeStep {
    fn from(val: BrokenspokeStep) -> Self {
        match val {
            BrokenspokeStep::Analysis => sea_orm_active_enums::BrokenspokeStep::Analysis,
            BrokenspokeStep::Cleanup => sea_orm_active_enums::BrokenspokeStep::Cleanup,
            BrokenspokeStep::Setup => sea_orm_active_enums::BrokenspokeStep::Setup,
            BrokenspokeStep::SqsMessage => sea_orm_active_enums::BrokenspokeStep::SqsMessage,
        }
    }
}

impl FromStr for BrokenspokeStep {
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

impl From<sea_orm_active_enums::BnaRegion> for BnaRegion {
    fn from(value: sea_orm_active_enums::BnaRegion) -> Self {
        match value {
            sea_orm_active_enums::BnaRegion::MidAtlantic => Self::MidAtlantic,
            sea_orm_active_enums::BnaRegion::NewEngland => Self::NewEngland,
            sea_orm_active_enums::BnaRegion::Midwest => Self::Midwest,
            sea_orm_active_enums::BnaRegion::Mountain => Self::Mountain,
            sea_orm_active_enums::BnaRegion::Pacific => Self::Pacific,
            sea_orm_active_enums::BnaRegion::South => Self::South,
        }
    }
}

impl From<BnaRegion> for sea_orm_active_enums::BnaRegion {
    fn from(value: BnaRegion) -> Self {
        match value {
            BnaRegion::MidAtlantic => sea_orm_active_enums::BnaRegion::MidAtlantic,
            BnaRegion::NewEngland => sea_orm_active_enums::BnaRegion::NewEngland,
            BnaRegion::Midwest => sea_orm_active_enums::BnaRegion::Midwest,
            BnaRegion::Mountain => sea_orm_active_enums::BnaRegion::Mountain,
            BnaRegion::Pacific => sea_orm_active_enums::BnaRegion::Pacific,
            BnaRegion::South => sea_orm_active_enums::BnaRegion::South,
        }
    }
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
