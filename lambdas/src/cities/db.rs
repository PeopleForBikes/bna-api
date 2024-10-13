use entity::{census, city};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};

pub async fn fetch_city(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
) -> Result<Option<city::Model>, sea_orm::DbErr> {
    city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
        .one(db)
        .await
}

pub async fn fetch_cities(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<city::Model>), sea_orm::DbErr> {
    let select = city::Entity::find();
    let count = select
        .clone()
        .select_only()
        .column_as(city::Column::Id.count(), "count")
        .count(db)
        .await?;
    let models = select.paginate(db, page_size).fetch_page(page).await?;
    Ok((count, models))
}

pub async fn fetch_city_censuses(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<(city::Model, Option<census::Model>)>), sea_orm::DbErr> {
    let select =
        city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
            .find_also_related(census::Entity);
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}
