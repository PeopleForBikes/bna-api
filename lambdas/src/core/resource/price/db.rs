use crate::core::resource::schema::OrderDirection;
use entity::fargate_price;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect,
};

pub async fn fetch_fargate_price(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<fargate_price::Model>, sea_orm::DbErr> {
    fargate_price::Entity::find_by_id(id).one(db).await
}

pub async fn fetch_fargate_prices(
    db: &DatabaseConnection,
    sort_direction: OrderDirection,
    sort_by: &str,
    latest: bool,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<fargate_price::Model>), sea_orm::DbErr> {
    let sort_column = match sort_by {
        "per_second" => fargate_price::Column::PerSecond,
        _ => fargate_price::Column::CreatedAt,
    };

    let mut select = fargate_price::Entity::find();
    let count = select
        .clone()
        .select_only()
        .column_as(fargate_price::Column::Id.count(), "count")
        .count(db)
        .await?;

    // Build the filter.
    select = match latest {
        true => select.order_by(sort_column, sea_orm::Order::Desc).limit(1),
        false => select.order_by(sort_column, sort_direction.into()),
    };

    // Retrieve the models.
    let models = match latest {
        true => select.paginate(db, 1).fetch_page(0).await?,
        false => select.paginate(db, page_size).fetch_page(page).await?,
    };
    Ok((count, models))
}
