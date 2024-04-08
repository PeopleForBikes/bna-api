use crate::entities::{brokenspoke_pipeline, sea_orm_active_enums};
use sea_orm::{
    prelude::{Json, TimeDateTimeWithTimeZone, Uuid},
    ActiveValue, IntoActiveModel,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrokenspokePipelinePost {
    pub state: Option<sea_orm_active_enums::BrokenspokeState>,
    pub state_machine_id: Uuid,
    pub scheduled_trigger_id: Option<Uuid>,
    pub sqs_message: Option<Json>,
    pub neon_branch_id: Option<String>,
    pub fargate_task_arn: Option<String>,
    pub s3_bucket: Option<String>,
    pub start_time: TimeDateTimeWithTimeZone,
    pub end_time: Option<TimeDateTimeWithTimeZone>,
    pub torn_down: Option<bool>,
    pub result_posted: Option<bool>,
}

impl IntoActiveModel<brokenspoke_pipeline::ActiveModel> for BrokenspokePipelinePost {
    fn into_active_model(self) -> brokenspoke_pipeline::ActiveModel {
        brokenspoke_pipeline::ActiveModel {
            state: ActiveValue::Set(self.state),
            state_machine_id: ActiveValue::Set(self.state_machine_id),
            sqs_message: ActiveValue::Set(self.sqs_message),
            neon_branch_id: ActiveValue::Set(self.neon_branch_id),
            fargate_task_arn: ActiveValue::Set(self.fargate_task_arn),
            s3_bucket: ActiveValue::Set(self.s3_bucket),
            scheduled_trigger_id: ActiveValue::Set(self.scheduled_trigger_id),
            start_time: ActiveValue::Set(self.start_time),
            end_time: ActiveValue::Set(self.end_time),
            torn_down: ActiveValue::Set(self.torn_down),
            results_posted: ActiveValue::Set(self.result_posted),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrokenspokePipelinePatch {
    pub state: Option<Option<sea_orm_active_enums::BrokenspokeState>>,
    pub scheduled_trigger_id: Option<Option<Uuid>>,
    pub sqs_message: Option<Option<Json>>,
    pub neon_branch_id: Option<Option<String>>,
    pub fargate_task_arn: Option<Option<String>>,
    pub s3_bucket: Option<Option<String>>,
    pub start_time: Option<Option<TimeDateTimeWithTimeZone>>,
    pub end_time: Option<Option<TimeDateTimeWithTimeZone>>,
    pub torn_down: Option<Option<bool>>,
    pub result_posted: Option<Option<bool>>,
}

impl IntoActiveModel<brokenspoke_pipeline::ActiveModel> for BrokenspokePipelinePatch {
    fn into_active_model(self) -> brokenspoke_pipeline::ActiveModel {
        brokenspoke_pipeline::ActiveModel {
            state_machine_id: ActiveValue::NotSet,
            state: self.state.map_or(ActiveValue::NotSet, ActiveValue::Set),
            sqs_message: self
                .sqs_message
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            neon_branch_id: self
                .neon_branch_id
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            fargate_task_arn: self
                .fargate_task_arn
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            s3_bucket: self.s3_bucket.map_or(ActiveValue::NotSet, ActiveValue::Set),
            scheduled_trigger_id: self
                .scheduled_trigger_id
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            start_time: ActiveValue::NotSet,
            end_time: self.end_time.map_or(ActiveValue::NotSet, ActiveValue::Set),
            torn_down: self.torn_down.map_or(ActiveValue::NotSet, ActiveValue::Set),
            results_posted: self
                .result_posted
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
        }
    }
}
