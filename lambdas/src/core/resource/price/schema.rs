//! Describes the Price schemas.
use chrono::DateTime;
use sea_orm::prelude::Decimal;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
#[schema(description = "A Fargate price used to estimate the cost of an analysis")]
pub(crate) struct FargatePrice {
    /// Identifier of the Fargate Price rate used to compute the cost of the pipeline run
    #[schema(examples("1"))]
    id: i32,
    /// Cost to run Fargate for 1 second
    #[schema(examples("0.0023"))]
    per_second: Decimal,
    /// Creation date
    created_at: DateTime<chrono::FixedOffset>,
}

impl From<entity::fargate_price::Model> for FargatePrice {
    fn from(value: entity::fargate_price::Model) -> Self {
        Self {
            id: value.id,
            per_second: value.per_second,
            created_at: value.created_at,
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(description = "A collection of Fargate prices.")]
pub(crate) struct FargatePrices(Vec<FargatePrice>);

impl From<Vec<entity::fargate_price::Model>> for FargatePrices {
    fn from(value: Vec<entity::fargate_price::Model>) -> Self {
        let prices = value
            .into_iter()
            .map(FargatePrice::from)
            .collect::<Vec<FargatePrice>>();
        Self(prices)
    }
}
