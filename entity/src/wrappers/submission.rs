use crate::entities::{sea_orm_active_enums, submission};
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

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
            created_at: ActiveValue::NotSet,
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
            created_at: ActiveValue::NotSet,
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
            created_at: ActiveValue::NotSet,
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
            created_at: ActiveValue::NotSet,
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
            created_at: ActiveValue::NotSet,
        };
        assert_eq!(active_model, expected);
    }
}
