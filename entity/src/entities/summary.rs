//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "summary")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bna_uuid: Uuid,
    pub city_id: Uuid,
    pub created_at: TimeDateTimeWithTimeZone,
    #[sea_orm(column_type = "Double")]
    pub score: f64,
    pub version: String,
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
    #[sea_orm(has_many = "super::core_services::Entity")]
    CoreServices,
    #[sea_orm(has_many = "super::features::Entity")]
    Features,
    #[sea_orm(has_many = "super::infrastructure::Entity")]
    Infrastructure,
    #[sea_orm(has_many = "super::opportunity::Entity")]
    Opportunity,
    #[sea_orm(has_many = "super::recreation::Entity")]
    Recreation,
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}

impl Related<super::core_services::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoreServices.def()
    }
}

impl Related<super::features::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Features.def()
    }
}

impl Related<super::infrastructure::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Infrastructure.def()
    }
}

impl Related<super::opportunity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Opportunity.def()
    }
}

impl Related<super::recreation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recreation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
