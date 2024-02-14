use crate::entities::{brokenspoke_pipeline, sea_orm_active_enums, submission};
use sea_orm::{
    prelude::{Json, TimeDateTimeWithTimeZone, Uuid},
    ActiveValue, IntoActiveModel,
};
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionPost {
    pub first_name: String,
    pub last_name: String,
    pub title: Option<String>,
    pub organization: Option<String>,
    pub email: String,
    pub country: String,
    pub city: String,
    pub region: Option<String>,
    pub fips_code: String,
    pub consent: bool,
    pub status: Option<sea_orm_active_enums::ApprovalStatus>,
}

impl IntoActiveModel<submission::ActiveModel> for SubmissionPost {
    fn into_active_model(self) -> submission::ActiveModel {
        submission::ActiveModel {
            id: ActiveValue::NotSet,
            first_name: ActiveValue::Set(self.first_name),
            last_name: ActiveValue::Set(self.last_name),
            title: ActiveValue::Set(self.title),
            organization: ActiveValue::Set(self.organization),
            email: ActiveValue::Set(self.email),
            country: ActiveValue::Set(self.country),
            city: ActiveValue::Set(self.city),
            region: ActiveValue::Set(self.region),
            fips_code: ActiveValue::Set(self.fips_code),
            consent: ActiveValue::Set(self.consent),
            status: self.status.map_or(ActiveValue::NotSet, ActiveValue::Set),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionPatch {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub title: Option<Option<String>>,
    pub organization: Option<Option<String>>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<Option<String>>,
    pub fips_code: Option<String>,
    pub consent: Option<bool>,
    pub status: Option<sea_orm_active_enums::ApprovalStatus>,
}

impl IntoActiveModel<submission::ActiveModel> for SubmissionPatch {
    fn into_active_model(self) -> submission::ActiveModel {
        submission::ActiveModel {
            id: ActiveValue::NotSet,
            first_name: self
                .first_name
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            last_name: self.last_name.map_or(ActiveValue::NotSet, ActiveValue::Set),
            title: self.title.map_or(ActiveValue::NotSet, ActiveValue::Set),
            organization: self
                .organization
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            email: self.email.map_or(ActiveValue::NotSet, ActiveValue::Set),
            country: self.country.map_or(ActiveValue::NotSet, ActiveValue::Set),
            city: self.city.map_or(ActiveValue::NotSet, ActiveValue::Set),
            region: self.region.map_or(ActiveValue::NotSet, ActiveValue::Set),
            fips_code: self.fips_code.map_or(ActiveValue::NotSet, ActiveValue::Set),
            consent: self.consent.map_or(ActiveValue::NotSet, ActiveValue::Set),
            status: self.status.map_or(ActiveValue::NotSet, ActiveValue::Set),
        }
    }
}

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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submission_post_into_active_model_full() {
        let first_name = "John".to_string();
        let last_name = "Doe".to_string();
        let title = Some("Director".to_owned());
        let organization = Some("ACME".to_string());
        let email = "john.doe@acme.org".to_string();
        let country = "usa".to_string();
        let city = "austin".to_string();
        let region = Some("texas".to_string());
        let fips_code = "0123456".to_string();
        let consent = true;
        let status = None;
        let wrapper = SubmissionPost {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            title: title.clone(),
            organization: organization.clone(),
            email: email.clone(),
            country: country.clone(),
            city: city.clone(),
            region: region.clone(),
            fips_code: fips_code.clone(),
            consent,
            status: status.clone(),
        };
        let active_model = wrapper.into_active_model();
        let expected = submission::ActiveModel {
            id: ActiveValue::NotSet,
            first_name: ActiveValue::Set(first_name),
            last_name: ActiveValue::Set(last_name),
            title: ActiveValue::Set(title),
            organization: ActiveValue::Set(organization),
            email: ActiveValue::Set(email),
            country: ActiveValue::Set(country),
            city: ActiveValue::Set(city),
            region: ActiveValue::Set(region),
            fips_code: ActiveValue::Set(fips_code),
            consent: ActiveValue::Set(consent),
            status: ActiveValue::NotSet,
        };
        assert_eq!(active_model, expected);
    }

    #[test]
    fn test_submission_post_into_active_model_required_only() {
        let first_name = "John".to_string();
        let last_name = "Doe".to_string();
        let title = None;
        let organization = None;
        let email = "john.doe@acme.org".to_string();
        let country = "usa".to_string();
        let city = "austin".to_string();
        let region = None;
        let fips_code = "0123456".to_string();
        let consent = true;
        let status = Some(sea_orm_active_enums::ApprovalStatus::Approved);
        let wrapper = SubmissionPost {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            title: title.clone(),
            organization: organization.clone(),
            email: email.clone(),
            country: country.clone(),
            city: city.clone(),
            region: region.clone(),
            fips_code: fips_code.clone(),
            consent,
            status: status.clone(),
        };
        let active_model = wrapper.into_active_model();
        let expected = submission::ActiveModel {
            id: ActiveValue::NotSet,
            first_name: ActiveValue::Set(first_name),
            last_name: ActiveValue::Set(last_name),
            title: ActiveValue::Set(title),
            organization: ActiveValue::Set(organization),
            email: ActiveValue::Set(email),
            country: ActiveValue::Set(country),
            city: ActiveValue::Set(city),
            region: ActiveValue::Set(region),
            fips_code: ActiveValue::Set(fips_code),
            consent: ActiveValue::Set(consent),
            status: ActiveValue::Set(sea_orm_active_enums::ApprovalStatus::Approved),
        };
        assert_eq!(active_model, expected);
    }

    #[test]
    fn test_submission_patch_into_model() {
        let id = 42;
        let first_name = "John".to_string();
        let wrapper = SubmissionPatch {
            first_name: Some(first_name.clone()),
            last_name: None,
            title: None,
            organization: None,
            email: None,
            country: None,
            city: None,
            region: None,
            fips_code: None,
            consent: None,
            status: None,
        };
        let active_model = wrapper.into_active_model();
        let expected = submission::ActiveModel {
            id: ActiveValue::Unchanged(id),
            first_name: ActiveValue::Set(first_name),
            last_name: ActiveValue::NotSet,
            title: ActiveValue::NotSet,
            organization: ActiveValue::NotSet,
            email: ActiveValue::NotSet,
            country: ActiveValue::NotSet,
            city: ActiveValue::NotSet,
            region: ActiveValue::NotSet,
            fips_code: ActiveValue::NotSet,
            consent: ActiveValue::NotSet,
            status: ActiveValue::NotSet,
        };
        assert_eq!(active_model, expected);
    }
}
