//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ranking")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ranking_id: i32,
    pub city_id: Uuid,
    pub country: String,
    pub country_size: i32,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub global: i32,
    pub size: i32,
    pub state: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::city::Entity",
        from = "Column::CityId",
        to = "super::city::Column::CityId",
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
