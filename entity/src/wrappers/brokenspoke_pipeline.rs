use crate::entities::{brokenspoke_pipeline, sea_orm_active_enums};
use sea_orm::{
    prelude::{Decimal, Json, TimeDateTimeWithTimeZone, Uuid},
    ActiveValue, IntoActiveModel,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrokenspokePipelinePost {
    pub cost: Option<Decimal>,
    pub end_time: Option<TimeDateTimeWithTimeZone>,
    pub fargate_task_arn: Option<String>,
    pub result_posted: Option<bool>,
    pub s3_bucket: Option<String>,
    pub sqs_message: Option<Json>,
    pub start_time: TimeDateTimeWithTimeZone,
    pub state_machine_id: Uuid,
    pub step: Option<sea_orm_active_enums::BrokenspokeStep>,
    pub torn_down: Option<bool>,
}

impl IntoActiveModel<brokenspoke_pipeline::ActiveModel> for BrokenspokePipelinePost {
    fn into_active_model(self) -> brokenspoke_pipeline::ActiveModel {
        brokenspoke_pipeline::ActiveModel {
            cost: ActiveValue::Set(self.cost),
            end_time: ActiveValue::Set(self.end_time),
            fargate_task_arn: ActiveValue::Set(self.fargate_task_arn),
            results_posted: ActiveValue::Set(self.result_posted),
            s3_bucket: ActiveValue::Set(self.s3_bucket),
            sqs_message: ActiveValue::Set(self.sqs_message),
            start_time: ActiveValue::Set(self.start_time),
            state_machine_id: ActiveValue::Set(self.state_machine_id),
            step: ActiveValue::Set(self.step),
            torn_down: ActiveValue::Set(self.torn_down),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrokenspokePipelinePatch {
    pub cost: Option<Option<Decimal>>,
    pub end_time: Option<Option<TimeDateTimeWithTimeZone>>,
    pub fargate_task_arn: Option<Option<String>>,
    pub neon_branch_id: Option<Option<String>>,
    pub result_posted: Option<Option<bool>>,
    pub s3_bucket: Option<Option<String>>,
    pub scheduled_trigger_id: Option<Option<Uuid>>,
    pub sqs_message: Option<Option<Json>>,
    pub start_time: Option<Option<TimeDateTimeWithTimeZone>>,
    pub state: Option<Option<sea_orm_active_enums::BrokenspokeStep>>,
    pub torn_down: Option<Option<bool>>,
}

impl IntoActiveModel<brokenspoke_pipeline::ActiveModel> for BrokenspokePipelinePatch {
    fn into_active_model(self) -> brokenspoke_pipeline::ActiveModel {
        brokenspoke_pipeline::ActiveModel {
            state_machine_id: ActiveValue::NotSet,
            cost: self.cost.map_or(ActiveValue::NotSet, ActiveValue::Set),
            step: self.state.map_or(ActiveValue::NotSet, ActiveValue::Set),
            sqs_message: self
                .sqs_message
                .map_or(ActiveValue::NotSet, ActiveValue::Set),

            fargate_task_arn: self
                .fargate_task_arn
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            s3_bucket: self.s3_bucket.map_or(ActiveValue::NotSet, ActiveValue::Set),
            start_time: ActiveValue::NotSet,
            end_time: self.end_time.map_or(ActiveValue::NotSet, ActiveValue::Set),
            torn_down: self.torn_down.map_or(ActiveValue::NotSet, ActiveValue::Set),
            results_posted: self
                .result_posted
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
        }
    }
}
