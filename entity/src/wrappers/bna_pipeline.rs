use crate::entities::bna_pipeline;
use sea_orm::{
    prelude::{DateTimeWithTimeZone, Decimal, Json, Uuid},
    ActiveValue, IntoActiveModel,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BNAPipelinePost {
    pub cost: Option<Decimal>,
    pub end_time: Option<DateTimeWithTimeZone>,
    pub fargate_price_id: Option<i32>,
    pub fargate_task_arn: Option<String>,
    pub result_posted: Option<bool>,
    pub s3_bucket: Option<String>,
    pub sqs_message: Option<Json>,
    pub start_time: DateTimeWithTimeZone,
    pub state_machine_id: Uuid,
    pub step: String,
    pub torn_down: Option<bool>,
}

impl IntoActiveModel<bna_pipeline::ActiveModel> for BNAPipelinePost {
    fn into_active_model(self) -> bna_pipeline::ActiveModel {
        bna_pipeline::ActiveModel {
            cost: ActiveValue::Set(self.cost),
            end_time: ActiveValue::Set(self.end_time),
            fargate_price_id: ActiveValue::Set(self.fargate_price_id),
            fargate_task_arn: ActiveValue::Set(self.fargate_task_arn),
            s3_bucket: ActiveValue::Set(self.s3_bucket),
            sqs_message: ActiveValue::Set(self.sqs_message),
            start_time: ActiveValue::Set(self.start_time),
            state_machine_id: ActiveValue::Set(self.state_machine_id),
            status: ActiveValue::Set("Pending".to_string()),
            step: ActiveValue::Set(self.step),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BNAPipelinePatch {
    pub cost: Option<Option<Decimal>>,
    pub end_time: Option<Option<DateTimeWithTimeZone>>,
    pub fargate_price_id: Option<Option<i32>>,
    pub fargate_task_arn: Option<Option<String>>,
    pub result_posted: Option<Option<bool>>,
    pub s3_bucket: Option<Option<String>>,
    pub sqs_message: Option<Option<Json>>,
    pub start_time: Option<Option<DateTimeWithTimeZone>>,
    pub status: Option<String>,
    pub step: Option<String>,
    pub torn_down: Option<Option<bool>>,
}

impl IntoActiveModel<bna_pipeline::ActiveModel> for BNAPipelinePatch {
    fn into_active_model(self) -> bna_pipeline::ActiveModel {
        bna_pipeline::ActiveModel {
            state_machine_id: ActiveValue::NotSet,
            cost: self.cost.map_or(ActiveValue::NotSet, ActiveValue::Set),
            step: self.step.map_or(ActiveValue::NotSet, ActiveValue::Set),
            sqs_message: self
                .sqs_message
                .map_or(ActiveValue::NotSet, ActiveValue::Set),

            fargate_task_arn: self
                .fargate_task_arn
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            s3_bucket: self.s3_bucket.map_or(ActiveValue::NotSet, ActiveValue::Set),
            start_time: ActiveValue::NotSet,
            end_time: self.end_time.map_or(ActiveValue::NotSet, ActiveValue::Set),
            fargate_price_id: self
                .fargate_price_id
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            status: self.status.map_or(ActiveValue::NotSet, ActiveValue::Set),
        }
    }
}
