//! Describes the Pipeline schemas.
use std::{fmt::Display, str::FromStr};

use chrono::DateTime;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use utoipa::{
    openapi::schema::{Object, ObjectBuilder, OneOfBuilder, Schema, SchemaType, Type},
    ToSchema,
};
use uuid::Uuid;

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) enum PipelineStatus {
    Completed,
    Pending,
    Processing,
}

impl FromStr for PipelineStatus {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for PipelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct BnaPipeline {
    /// Cost of an analysis in USD
    #[schema(examples("6.8941"))]
    cost: Option<Decimal>,
    /// End time
    end_time: Option<DateTime<chrono::FixedOffset>>,
    /// Fargate price identifier used to compute the cost
    fargate_price_id: Option<i32>,
    /// ARN of the Fargate task that performed the analysis
    #[schema(examples(
        "arn:aws:ecs:us-west-2:123456789012:task/bna/29f979fc9fca402d94b014aa23d2f6e0"
    ))]
    fargate_task_arn: Option<String>,
    /// Path of the S3 bucket where the results were stored
    #[schema(examples("united states/new mexico/santa rosa/24.05.4"))]
    s3_bucket: Option<String>,
    /// Copy of the JSON message that was sent for processing
    #[schema(schema_with = sqs_message_schema)]
    sqs_message: Option<Map<String, Value>>,
    /// Start time
    start_time: DateTime<chrono::FixedOffset>,
    /// Pipeline identifier
    /// This is the ID of the AWS state machine that was used to run the pipeline
    state_machine_id: Uuid,
    /// Pipeline status
    status: PipelineStatus,
    /// Last pipeline step that was completed
    step: BnaPipelineStep,
}

impl From<entity::bna_pipeline::Model> for BnaPipeline {
    fn from(value: entity::bna_pipeline::Model) -> Self {
        Self {
            cost: value.cost,
            end_time: value.end_time,
            fargate_price_id: value.fargate_price_id,
            fargate_task_arn: value.fargate_task_arn,
            s3_bucket: value.s3_bucket,
            sqs_message: value.sqs_message.and_then(decode_sqs_message),
            start_time: value.start_time,
            state_machine_id: value.state_machine_id,
            status: PipelineStatus::from_str(&value.status).expect("a valid status"),
            step: BnaPipelineStep::from_str(&value.step).expect("a valid step"),
        }
    }
}

/// Documents the SQS message of a request: an optional JSON object, or a string
/// holding one.
fn sqs_message_request_schema() -> Schema {
    let one_of = OneOfBuilder::new()
        .item(sqs_message_schema())
        .item(
            ObjectBuilder::new()
                .schema_type(Type::String)
                .description(Some("A string holding a JSON object"))
                .examples([json!(
                    "{\"country\": \"United States\", \"city\": \"santa rosa\"}"
                )]),
        )
        .description(Some(
            "Copy of the JSON message that was sent for processing: a JSON object, \
            or a string holding a JSON object. It is stored and returned as a JSON \
            object.",
        ))
        .build();
    Schema::OneOf(one_of)
}

/// Documents the SQS message of a response: an optional JSON object.
fn sqs_message_schema() -> Object {
    ObjectBuilder::new()
        .schema_type(SchemaType::from_iter([Type::Object, Type::Null]))
        .description(Some(
            "Copy of the JSON message that was sent for processing",
        ))
        .examples([json!({
            "country": "United States",
            "city": "santa rosa",
            "region": "new mexico",
            "fips_code": "3570670"
        })])
        .build()
}

/// Returns the SQS message as a JSON object.
///
/// Messages sent as a string were stored double-encoded: a string holding a
/// JSON object is decoded. Any other value is not a message and is dropped.
fn decode_sqs_message(value: Value) -> Option<Map<String, Value>> {
    match value {
        Value::Object(object) => Some(object),
        Value::String(s) => serde_json::from_str::<Map<String, Value>>(&s).ok(),
        _ => None,
    }
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct BnaPipelinePost {
    /// Cost of an analysis in USD
    #[schema(examples("6.8941"))]
    cost: Option<Decimal>,
    /// End time
    end_time: Option<DateTime<chrono::FixedOffset>>,
    /// Fargate price identifier used to compute the cost
    fargate_price_id: Option<i32>,
    /// ARN of the Fargate task that performed the analysis
    #[schema(examples(
        "arn:aws:ecs:us-west-2:123456789012:task/bna/29f979fc9fca402d94b014aa23d2f6e0"
    ))]
    fargate_task_arn: Option<String>,
    /// Path of the S3 bucket where the results were stored
    #[schema(examples("united states/new mexico/santa rosa/24.05.4"))]
    s3_bucket: Option<String>,
    /// Copy of the JSON message that was sent for processing
    #[schema(schema_with = sqs_message_request_schema)]
    sqs_message: Option<Map<String, Value>>,
    /// Start time
    start_time: Option<DateTime<chrono::FixedOffset>>,
    /// Pipeline identifier
    /// This is the ID of the AWS state machine that was used to run the pipeline
    state_machine_id: Uuid,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct BnaPipelinePatch {
    /// Cost of an analysis in USD
    #[schema(examples("6.8941"))]
    cost: Option<Decimal>,
    /// End time
    end_time: Option<DateTime<chrono::FixedOffset>>,
    /// Fargate price identifier used to compute the cost
    fargate_price_id: Option<i32>,
    /// ARN of the Fargate task that performed the analysis
    #[schema(examples(
        "arn:aws:ecs:us-west-2:123456789012:task/bna/29f979fc9fca402d94b014aa23d2f6e0"
    ))]
    fargate_task_arn: Option<String>,
    /// Path of the S3 bucket where the results were stored
    #[schema(examples("united states/new mexico/santa rosa/24.05.4"))]
    s3_bucket: Option<String>,
    /// Copy of the JSON message that was sent for processing
    #[schema(schema_with = sqs_message_request_schema)]
    sqs_message: Option<Map<String, Value>>,
    /// Start time
    start_time: Option<DateTime<chrono::FixedOffset>>,
    /// Pipeline status
    #[schema(nullable = false)]
    status: Option<PipelineStatus>,
    /// Last pipeline step that was completed
    #[schema(nullable = false)]
    step: Option<BnaPipelineStep>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) enum BnaPipelineStep {
    Setup,
    Analysis,
    Save,
    Cleanup,
}

impl FromStr for BnaPipelineStep {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}

impl Display for BnaPipelineStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub(crate) struct BnaPipelines(Vec<BnaPipeline>);

#[cfg(test)]
mod tests {
    use super::*;

    fn utrecht() -> Option<Map<String, Value>> {
        json!({"city": "Utrecht"}).as_object().cloned()
    }

    #[test]
    fn test_decode_sqs_message_object() {
        assert_eq!(decode_sqs_message(json!({"city": "Utrecht"})), utrecht());
    }

    #[test]
    fn test_decode_sqs_message_double_encoded() {
        assert_eq!(
            decode_sqs_message(json!("{\"city\":\"Utrecht\"}")),
            utrecht()
        );
    }

    #[test]
    fn test_decode_sqs_message_not_an_object() {
        assert_eq!(decode_sqs_message(json!("Utrecht")), None);
    }
}
