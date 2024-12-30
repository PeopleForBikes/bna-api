//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "fargate_price")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub per_second: Decimal,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bna_pipeline::Entity")]
    BnaPipeline,
}

impl Related<super::bna_pipeline::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BnaPipeline.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
