use entity::{census, city, country, state_region_crosswalk, summary};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};

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

pub async fn fetch_cities_censuses(
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

pub async fn fetch_cities_ratings(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<(city::Model, Option<summary::Model>)>), sea_orm::DbErr> {
    let select =
        city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
            .find_also_related(summary::Entity);
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}

pub async fn fetch_country(
    db: &DatabaseConnection,
    country: &str,
) -> Result<Option<country::Model>, sea_orm::DbErr> {
    country::Entity::find()
        .filter(country::Column::Name.eq(country))
        .one(db)
        .await
}

pub async fn fetch_state_region_crosswalk(
    db: &DatabaseConnection,
    state: &str,
) -> Result<Option<state_region_crosswalk::Model>, sea_orm::DbErr> {
    state_region_crosswalk::Entity::find()
        .filter(state_region_crosswalk::Column::State.eq(state))
        .one(db)
        .await
}

pub async fn insert_cities(
    db: &DatabaseConnection,
    city: entity::city::ActiveModel,
) -> Result<city::Model, sea_orm::DbErr> {
    city.insert(db).await
}
