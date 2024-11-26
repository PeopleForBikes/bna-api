use crate::census;
use sea_orm::{prelude::Uuid, ActiveValue, IntoActiveModel};
use serde::Deserialize;

pub struct CensusPost {
    pub city_id: Uuid,
    pub fips_code: String,
    pub pop_size: i32,
    pub population: i32,
}

impl IntoActiveModel<census::ActiveModel> for CensusPost {
    fn into_active_model(self) -> census::ActiveModel {
        census::ActiveModel {
            id: ActiveValue::NotSet,
            city_id: ActiveValue::Set(self.city_id),
            created_at: ActiveValue::NotSet,
            fips_code: ActiveValue::Set(self.fips_code),
            pop_size: ActiveValue::Set(self.pop_size),
            population: ActiveValue::Set(self.population),
        }
    }
}

#[derive(Deserialize)]
pub struct CensusFromCityPost {
    pub fips_code: String,
    pub pop_size: i32,
    pub population: i32,
}

impl IntoActiveModel<census::ActiveModel> for CensusFromCityPost {
    fn into_active_model(self) -> census::ActiveModel {
        census::ActiveModel {
            id: ActiveValue::NotSet,
            city_id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            fips_code: ActiveValue::Set(self.fips_code),
            pop_size: ActiveValue::Set(self.pop_size),
            population: ActiveValue::Set(self.population),
        }
    }
}

pub struct CensusPatch {
    pub city_id: Option<Uuid>,
    pub fips_code: Option<String>,
    pub pop_size: Option<i32>,
    pub population: Option<i32>,
}

impl IntoActiveModel<census::ActiveModel> for CensusPatch {
    fn into_active_model(self) -> census::ActiveModel {
        census::ActiveModel {
            id: ActiveValue::NotSet,
            city_id: self.city_id.map_or(ActiveValue::NotSet, ActiveValue::Set),
            created_at: ActiveValue::NotSet,
            fips_code: self.fips_code.map_or(ActiveValue::NotSet, ActiveValue::Set),
            pop_size: self.pop_size.map_or(ActiveValue::NotSet, ActiveValue::Set),
            population: self
                .population
                .map_or(ActiveValue::NotSet, ActiveValue::Set),
        }
    }
}
