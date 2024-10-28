use entity::fargate_price;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect,
};

use crate::Sort;

pub async fn fetch_fargate_price(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<fargate_price::Model>, sea_orm::DbErr> {
    fargate_price::Entity::find_by_id(id).one(db).await
}

pub async fn fetch_fargate_prices(
    db: &DatabaseConnection,
    sort: Sort,
    latest: bool,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<fargate_price::Model>), sea_orm::DbErr> {
    let mut select = fargate_price::Entity::find();
    let count = select
        .clone()
        .select_only()
        .column_as(fargate_price::Column::Id.count(), "count")
        .count(db)
        .await?;

    // Build the filter.
    let created_at_column = entity::fargate_price::Column::CreatedAt;
    select = match (latest, sort) {
        (true, _) | (false, Sort::Desc) => select
            .order_by(created_at_column, sea_orm::Order::Desc)
            .limit(1),
        (false, Sort::Asc) => select.order_by(created_at_column, sea_orm::Order::Asc),
    };

    // Retrieve the models.
    let models = match latest {
        true => select.limit(1).paginate(db, 1).fetch_page(0).await?,
        false => select.paginate(db, page_size).fetch_page(page).await?,
    };
    Ok((count, models))
}
