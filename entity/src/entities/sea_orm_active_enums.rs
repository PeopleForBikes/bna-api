//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "approval_status")]
pub enum ApprovalStatus {
    #[sea_orm(string_value = "Approved")]
    Approved,
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "Rejected")]
    Rejected,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "brokenspoke_state")]
pub enum BrokenspokeState {
    #[sea_orm(string_value = "analysis")]
    Analysis,
    #[sea_orm(string_value = "cleanup")]
    Cleanup,
    #[sea_orm(string_value = "setup")]
    Setup,
    #[sea_orm(string_value = "sqs_message")]
    SqsMessage,
}
