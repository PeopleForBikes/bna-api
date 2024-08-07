//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "features")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bna_id: Uuid,
    #[sea_orm(column_type = "Double", nullable)]
    pub people: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub retail: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub transit: Option<f64>,
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
