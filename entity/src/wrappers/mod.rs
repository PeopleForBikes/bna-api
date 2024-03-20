pub mod bna;
pub mod brokenspoke_pipeline;
pub mod census;
pub mod city;
pub mod submission;

use crate::entities::sea_orm_active_enums;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
pub enum BrokenspokeState {
    SqsMessage,
    Setup,
    Analysis,
    Cleanup,
}

impl From<sea_orm_active_enums::BrokenspokeState> for BrokenspokeState {
    fn from(value: sea_orm_active_enums::BrokenspokeState) -> Self {
        match value {
            sea_orm_active_enums::BrokenspokeState::Analysis => Self::Analysis,
            sea_orm_active_enums::BrokenspokeState::Cleanup => Self::Cleanup,
            sea_orm_active_enums::BrokenspokeState::Setup => Self::Setup,
            sea_orm_active_enums::BrokenspokeState::SqsMessage => Self::SqsMessage,
        }
    }
}

impl From<BrokenspokeState> for sea_orm_active_enums::BrokenspokeState {
    fn from(val: BrokenspokeState) -> Self {
        match val {
            BrokenspokeState::Analysis => sea_orm_active_enums::BrokenspokeState::Analysis,
            BrokenspokeState::Cleanup => sea_orm_active_enums::BrokenspokeState::Cleanup,
            BrokenspokeState::Setup => sea_orm_active_enums::BrokenspokeState::Setup,
            BrokenspokeState::SqsMessage => sea_orm_active_enums::BrokenspokeState::SqsMessage,
        }
    }
}

impl FromStr for BrokenspokeState {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
