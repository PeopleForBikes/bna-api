//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "summary")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub city_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub pop_size: i32,
    pub population: i32,
    pub residential_speed_limit_override: Option<i32>,
    #[sea_orm(column_type = "Double")]
    pub score: f64,
    pub version: String,
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
    #[sea_orm(has_one = "super::core_services::Entity")]
    CoreServices,
    #[sea_orm(has_one = "super::infrastructure::Entity")]
    Infrastructure,
    #[sea_orm(has_one = "super::opportunity::Entity")]
    Opportunity,
    #[sea_orm(has_one = "super::people::Entity")]
    People,
    #[sea_orm(has_one = "super::recreation::Entity")]
    Recreation,
    #[sea_orm(has_one = "super::retail::Entity")]
    Retail,
    #[sea_orm(has_one = "super::transit::Entity")]
    Transit,
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

impl Related<super::people::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::People.def()
    }
}

impl Related<super::recreation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recreation.def()
    }
}

impl Related<super::retail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Retail.def()
    }
}

impl Related<super::transit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
