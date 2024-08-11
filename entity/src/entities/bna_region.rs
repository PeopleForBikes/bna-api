//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "bna_region")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::state_region_crosswalk::Entity")]
    StateRegionCrosswalk,
}

impl Related<super::state_region_crosswalk::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StateRegionCrosswalk.def()
    }
}

impl Related<super::us_state::Entity> for Entity {
    fn to() -> RelationDef {
        super::state_region_crosswalk::Relation::UsState.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::state_region_crosswalk::Relation::BnaRegion
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
