use crate::{
    entities::bna_pipeline,
    wrappers::{BNAPipelineStatus, BNAPipelineStep},
};
use sea_orm::{
    prelude::{DateTimeWithTimeZone, Decimal, Json, Uuid},
    sqlx::types::chrono::{FixedOffset, Utc},
    ActiveValue, IntoActiveModel,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::Map;

/// A JSON object, as opposed to any JSON value.
pub type JsonObject = Map<String, Json>;

/// Reads an SQS message: a JSON object, or a string holding one.
///
/// The operator must be able to turn the stored message back into JSON, so a
/// string that does not hold a JSON object is rejected.
fn parse_sqs_message<E: Error>(value: Json) -> Result<JsonObject, E> {
    match value {
        Json::Object(object) => Ok(object),
        Json::String(s) => serde_json::from_str::<JsonObject>(&s)
            .map_err(|_| E::custom("expected a string holding a JSON object")),
        _ => Err(E::custom(
            "expected a JSON object, or a string holding a JSON object",
        )),
    }
}

/// Deserializes an optional SQS message, `null` meaning no message.
fn deserialize_sqs_message<'de, D>(deserializer: D) -> Result<Option<JsonObject>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<Json>::deserialize(deserializer)?
        .map(parse_sqs_message)
        .transpose()
}

/// Deserializes the SQS message of an update: absent leaves it unchanged
/// (through `#[serde(default)]`), and `null` clears it.
fn deserialize_sqs_message_patch<'de, D>(
    deserializer: D,
) -> Result<Option<Option<JsonObject>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_sqs_message(deserializer).map(Some)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BNAPipelinePost {
    pub cost: Option<Decimal>,
    pub end_time: Option<DateTimeWithTimeZone>,
    pub fargate_price_id: Option<i32>,
    pub fargate_task_arn: Option<String>,
    pub result_posted: Option<bool>,
    pub s3_bucket: Option<String>,
    #[serde(default, deserialize_with = "deserialize_sqs_message")]
    pub sqs_message: Option<JsonObject>,
    pub start_time: Option<DateTimeWithTimeZone>,
    pub state_machine_id: Uuid,
}

impl IntoActiveModel<bna_pipeline::ActiveModel> for BNAPipelinePost {
    fn into_active_model(self) -> bna_pipeline::ActiveModel {
        bna_pipeline::ActiveModel {
            cost: ActiveValue::Set(self.cost),
            end_time: ActiveValue::Set(self.end_time),
            fargate_price_id: ActiveValue::Set(self.fargate_price_id),
            fargate_task_arn: ActiveValue::Set(self.fargate_task_arn),
            s3_bucket: ActiveValue::Set(self.s3_bucket),
            sqs_message: ActiveValue::Set(self.sqs_message.map(Json::Object)),
            start_time: ActiveValue::Set(
                self.start_time.unwrap_or_else(|| {
                    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
                }),
            ),
            state_machine_id: ActiveValue::Set(self.state_machine_id),
            status: ActiveValue::Set("Pending".to_string()),
            step: ActiveValue::Set("Setup".to_string()),
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
    #[serde(default, deserialize_with = "deserialize_sqs_message_patch")]
    pub sqs_message: Option<Option<JsonObject>>,
    pub start_time: Option<DateTimeWithTimeZone>,
    pub status: Option<BNAPipelineStatus>,
    pub step: Option<BNAPipelineStep>,
    pub torn_down: Option<Option<bool>>,
}

impl IntoActiveModel<bna_pipeline::ActiveModel> for BNAPipelinePatch {
    fn into_active_model(self) -> bna_pipeline::ActiveModel {
        bna_pipeline::ActiveModel {
            state_machine_id: ActiveValue::NotSet,
            cost: self.cost.map_or(ActiveValue::NotSet, ActiveValue::Set),
            step: self
                .step
                .map_or(ActiveValue::NotSet, |s| ActiveValue::Set(s.to_string())),
            sqs_message: self.sqs_message.map_or(ActiveValue::NotSet, |v| {
                ActiveValue::Set(v.map(Json::Object))
            }),

            fargate_task_arn: self
                .fargate_task_arn
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            s3_bucket: self.s3_bucket.map_or(ActiveValue::NotSet, ActiveValue::Set),
            start_time: self
                .start_time
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            end_time: self.end_time.map_or(ActiveValue::NotSet, ActiveValue::Set),
            fargate_price_id: self
                .fargate_price_id
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            status: self
                .status
                .map_or(ActiveValue::NotSet, |s| ActiveValue::Set(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn post(sqs_message: Json) -> Result<BNAPipelinePost, serde_json::Error> {
        serde_json::from_value(json!({
            "state_machine_id": "00000000-0000-4000-8000-000000000000",
            "sqs_message": sqs_message,
        }))
    }

    fn patch(body: Json) -> Result<BNAPipelinePatch, serde_json::Error> {
        serde_json::from_value(body)
    }

    fn utrecht() -> JsonObject {
        json!({"city": "Utrecht"}).as_object().unwrap().clone()
    }

    #[test]
    fn test_sqs_message_object() {
        let wrapper = post(json!({"city": "Utrecht"})).unwrap();
        assert_eq!(wrapper.sqs_message, Some(utrecht()));
    }

    #[test]
    fn test_sqs_message_string_holding_an_object() {
        let wrapper = post(json!("{\"city\": \"Utrecht\"}")).unwrap();
        assert_eq!(wrapper.sqs_message, Some(utrecht()));
    }

    #[test]
    fn test_sqs_message_null() {
        assert_eq!(post(Json::Null).unwrap().sqs_message, None);
    }

    #[test]
    fn test_sqs_message_string_not_json() {
        assert!(post(json!("Utrecht")).is_err());
    }

    #[test]
    fn test_sqs_message_string_holding_an_array() {
        assert!(post(json!("[1, 2]")).is_err());
    }

    #[test]
    fn test_sqs_message_number() {
        assert!(post(json!(42)).is_err());
    }

    #[test]
    fn test_sqs_message_patch() {
        assert_eq!(patch(json!({})).unwrap().sqs_message, None);
        assert_eq!(
            patch(json!({"sqs_message": null})).unwrap().sqs_message,
            Some(None)
        );
        assert_eq!(
            patch(json!({"sqs_message": "{\"city\": \"Utrecht\"}"}))
                .unwrap()
                .sqs_message,
            Some(Some(utrecht()))
        );
    }
}
