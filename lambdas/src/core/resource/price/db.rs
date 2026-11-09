use crate::core::resource::schema::OrderDirection;
use entity::fargate_price;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};

pub async fn fetch_fargate_price(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<fargate_price::Model>, sea_orm::DbErr> {
    fargate_price::Entity::find_by_id(id).one(db).await
}

/// Fetches the latest Fargate price: the most recent one, the highest
/// identifier breaking a tie.
pub async fn fetch_latest_fargate_price(
    db: &DatabaseConnection,
) -> Result<Option<fargate_price::Model>, sea_orm::DbErr> {
    fargate_price::Entity::find()
        .order_by_desc(fargate_price::Column::CreatedAt)
        .order_by_desc(fargate_price::Column::Id)
        .one(db)
        .await
}

pub async fn fetch_fargate_prices(
    db: &DatabaseConnection,
    sort_direction: OrderDirection,
    sort_by: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<fargate_price::Model>), sea_orm::DbErr> {
    let sort_column = match sort_by {
        "per_second" => fargate_price::Column::PerSecond,
        _ => fargate_price::Column::CreatedAt,
    };

    let select = fargate_price::Entity::find();
    let count = select.clone().count(db).await?;
    let models = select
        .order_by(sort_column, sort_direction.into())
        .order_by_asc(fargate_price::Column::Id)
        .paginate(db, page_size)
        .fetch_page(page.saturating_sub(1))
        .await?;
    Ok((count, models))
}
