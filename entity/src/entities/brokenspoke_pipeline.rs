//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use super::sea_orm_active_enums::BrokenspokeStep;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "brokenspoke_pipeline")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub state_machine_id: Uuid,
    pub step: Option<BrokenspokeStep>,
    pub sqs_message: Option<Json>,
    pub fargate_task_arn: Option<String>,
    pub s3_bucket: Option<String>,
    pub start_time: TimeDateTimeWithTimeZone,
    pub end_time: Option<TimeDateTimeWithTimeZone>,
    pub torn_down: Option<bool>,
    pub results_posted: Option<bool>,
    pub cost: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
