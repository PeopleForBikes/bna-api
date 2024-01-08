use crate::entities::sea_orm_active_enums::ApprovalStatus;
use crate::entities::submission;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submission {
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
    pub status: Option<ApprovalStatus>,
}

impl IntoActiveModel<submission::ActiveModel> for Submission {
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
            status: ActiveValue::Set(self.status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submission_into_active_model_full() {
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
        let status = Some(ApprovalStatus::Pending);
        let wrapper = Submission {
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
            status: ActiveValue::Set(status),
        };
        assert_eq!(active_model, expected);
    }

    #[test]
    fn test_submission_into_active_model_required_only() {
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
        let status = Some(ApprovalStatus::Pending);
        let wrapper = Submission {
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
            status: ActiveValue::Set(status),
        };
        assert_eq!(active_model, expected);
    }
}
