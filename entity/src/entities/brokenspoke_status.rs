//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "brokenspoke_status")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::brokenspoke_pipeline::Entity")]
    BrokenspokePipeline,
}

impl Related<super::brokenspoke_pipeline::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BrokenspokePipeline.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
