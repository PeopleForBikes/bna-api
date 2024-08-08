use crate::city;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CityPost {
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub name: String,
    pub region: Option<String>,
    pub state: String,
    pub state_abbrev: Option<String>,
    pub speed_limit: Option<i32>,
}

impl IntoActiveModel<city::ActiveModel> for CityPost {
    fn into_active_model(self) -> city::ActiveModel {
        city::ActiveModel {
            id: ActiveValue::NotSet,
            country: ActiveValue::Set(self.country),
            latitude: ActiveValue::Set(self.latitude),
            longitude: ActiveValue::Set(self.longitude),
            name: ActiveValue::Set(self.name),
            region: ActiveValue::set(self.region),
            state: ActiveValue::Set(self.state),
            state_abbrev: ActiveValue::Set(self.state_abbrev),
            speed_limit: ActiveValue::Set(self.speed_limit),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CityPatch {
    pub country: Option<String>,
    pub latitude: Option<Option<f64>>,
    pub longitude: Option<Option<f64>>,
    pub name: Option<String>,
    pub region: Option<Option<String>>,
    pub state: Option<String>,
    pub state_abbrev: Option<Option<String>>,
    pub speed_limit: Option<Option<i32>>,
}

impl IntoActiveModel<city::ActiveModel> for CityPatch {
    fn into_active_model(self) -> city::ActiveModel {
        city::ActiveModel {
            id: ActiveValue::NotSet,
            country: self.country.map_or(ActiveValue::NotSet, ActiveValue::Set),
            latitude: self.latitude.map_or(ActiveValue::NotSet, ActiveValue::Set),
            longitude: self.longitude.map_or(ActiveValue::NotSet, ActiveValue::Set),
            name: self.name.map_or(ActiveValue::NotSet, ActiveValue::Set),
            region: self.region.map_or(ActiveValue::NotSet, ActiveValue::Set),
            state: self.state.map_or(ActiveValue::NotSet, ActiveValue::Set),
            state_abbrev: self
                .state_abbrev
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            speed_limit: self
                .speed_limit
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
