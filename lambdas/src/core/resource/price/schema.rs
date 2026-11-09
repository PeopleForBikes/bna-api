//! Describes the Price schemas.
use crate::{
    core::resource::schema::{deserialize_page, deserialize_page_size, OrderDirection},
    DEFAULT_PAGE_SIZE,
};
use chrono::DateTime;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

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

/// Parameters of the Fargate price list: pagination, sorting, and latest.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
pub(crate) struct PriceListParameters {
    /// The result page being returned, starting at 1. A page past the last one returns an empty list.
    #[param(minimum = 1, maximum = 65536, example = 5)]
    #[serde(default, deserialize_with = "deserialize_page")]
    page: Option<u64>,
    /// The number of items per page
    #[param(minimum = 1, maximum = 100, example = 25)]
    #[serde(default, deserialize_with = "deserialize_page_size")]
    page_size: Option<u64>,
    /// Field to sort the prices by
    #[param(inline, example = "created_at")]
    sort_by: Option<PriceSortBy>,
    #[param(example = "desc")]
    order_direction: Option<OrderDirection>,
    /// Only return the latest price
    #[param(example = "true")]
    latest: Option<bool>,
}

impl PriceListParameters {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1)
    }

    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(DEFAULT_PAGE_SIZE)
    }

    pub fn sort_by(&self) -> String {
        self.sort_by.clone().unwrap_or_default().to_string()
    }

    pub fn order_direction(&self) -> OrderDirection {
        self.order_direction.clone().unwrap_or_default()
    }

    pub fn latest(&self) -> bool {
        self.latest.unwrap_or_default()
    }
}

/// Fields the Fargate prices can be sorted by.
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PriceSortBy {
    PerSecond,
    #[default]
    CreatedAt,
}

impl std::fmt::Display for PriceSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_plain::to_string(&self).expect("cannot serialize value");
        write!(f, "{value}")
    }
}
