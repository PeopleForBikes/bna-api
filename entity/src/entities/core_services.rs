//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "core_services")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bna_id: Uuid,
    #[sea_orm(column_type = "Double", nullable)]
    pub dentists: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub doctors: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub grocery: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub hospitals: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub pharmacies: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub score: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub social_services: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::summary::Entity",
        from = "Column::BnaId",
        to = "super::summary::Column::BnaId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Summary,
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
