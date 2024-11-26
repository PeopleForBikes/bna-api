use entity::{census, city, country, state_region_crosswalk, submission, summary};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};

pub async fn fetch_city(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
) -> Result<Option<city::Model>, DbErr> {
    city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
        .one(db)
        .await
}

pub async fn fetch_cities(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<city::Model>), DbErr> {
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
) -> Result<(u64, Vec<(city::Model, Option<census::Model>)>), DbErr> {
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
) -> Result<(u64, Vec<(city::Model, Option<summary::Model>)>), DbErr> {
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
) -> Result<Option<country::Model>, DbErr> {
    country::Entity::find()
        .filter(country::Column::Name.eq(country))
        .one(db)
        .await
}

pub async fn fetch_state_region_crosswalk(
    db: &DatabaseConnection,
    state: &str,
) -> Result<Option<state_region_crosswalk::Model>, DbErr> {
    state_region_crosswalk::Entity::find()
        .filter(state_region_crosswalk::Column::State.eq(state))
        .one(db)
        .await
}

pub async fn fetch_cities_submission(
    db: &DatabaseConnection,
    submission_id: i32,
    status: Option<String>,
) -> Result<Option<submission::Model>, DbErr> {
    // Filter the query if needed.
    let mut conditions = Condition::all();
    if let Some(status) = status {
        conditions = conditions.add(entity::submission::Column::Status.eq(status))
    }

    // Select the submission.
    submission::Entity::find_by_id(submission_id)
        .filter(conditions)
        .one(db)
        .await
}

pub async fn fetch_cities_submissions(
    db: &DatabaseConnection,
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::submission::Model>), DbErr> {
    // Filter the query if needed.
    let mut conditions = Condition::all();
    if let Some(status) = status {
        conditions = conditions.add(entity::submission::Column::Status.eq(status))
    }

    // Select the submissions.
    let select = submission::Entity::find().filter(conditions);
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}
