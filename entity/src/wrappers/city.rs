use crate::city;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CityPost {
    pub country: String,
    pub fips_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub name: String,
    pub region: Option<String>,
    pub state: String,
    pub state_abbrev: Option<String>,
    pub residential_speed_limit: Option<i32>,
}

impl IntoActiveModel<city::ActiveModel> for CityPost {
    fn into_active_model(self) -> city::ActiveModel {
        city::ActiveModel {
            country: ActiveValue::Set(self.country),
            created_at: ActiveValue::NotSet,
            fips_code: ActiveValue::Set(self.fips_code),
            id: ActiveValue::NotSet,
            latitude: ActiveValue::Set(self.latitude),
            longitude: ActiveValue::Set(self.longitude),
            name: ActiveValue::Set(self.name),
            region: ActiveValue::set(self.region),
            residential_speed_limit: ActiveValue::Set(self.residential_speed_limit),
            state_abbrev: ActiveValue::Set(self.state_abbrev),
            state: ActiveValue::Set(self.state),
            updated_at: ActiveValue::NotSet,
        }
    }
}
