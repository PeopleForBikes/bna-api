//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "census")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub city_id: Uuid,
    pub created_at: TimeDateTimeWithTimeZone,
    pub fips_code: String,
    pub pop_size: i32,
    pub population: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::city::Entity",
        from = "Column::CityId",
        to = "super::city::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    City,
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
