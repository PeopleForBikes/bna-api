use entity::{city, country, state_region_crosswalk, submission, summary, Summary};
use sea_orm::{
    ColumnTrait, Condition, DatabaseBackend, DatabaseConnection, DbErr, EntityTrait,
    FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Statement,
};
use serde::Serialize;
use uuid::Uuid;

pub(crate) async fn fetch_city(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
) -> Result<Option<city::Model>, DbErr> {
    city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
        .one(db)
        .await
}

pub(crate) async fn fetch_cities(
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

pub(crate) async fn fetch_cities_ratings(
    db: &DatabaseConnection,
    country: &str,
    region: &str,
    name: &str,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<(city::Model, Option<summary::Model>)>), DbErr> {
    let select =
        city::Entity::find_by_id((country.to_string(), region.to_string(), name.to_string()))
            .find_also_related(summary::Entity)
            .order_by_desc(summary::Column::Version);
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}

pub(crate) async fn fetch_country(
    db: &DatabaseConnection,
    country: &str,
) -> Result<Option<country::Model>, DbErr> {
    country::Entity::find()
        .filter(country::Column::Name.eq(country))
        .one(db)
        .await
}

pub(crate) async fn fetch_state_region_crosswalk(
    db: &DatabaseConnection,
    state: &str,
) -> Result<Option<state_region_crosswalk::Model>, DbErr> {
    state_region_crosswalk::Entity::find()
        .filter(state_region_crosswalk::Column::State.eq(state))
        .one(db)
        .await
}

pub(crate) async fn fetch_cities_submission(
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

pub(crate) async fn fetch_cities_submissions(
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

#[derive(FromQueryResult, Serialize)]
struct SummaryID {
    id: Uuid,
}

pub(crate) async fn fetch_top_cities(
    db: &DatabaseConnection,
    year: i32,
    count: i32,
) -> Result<Option<Vec<(city::Model, summary::Model)>>, DbErr> {
    // Define a custom query to fetch the top N summary IDs of a specific year.
    let query = r#"
    WITH latest_scores AS (
    SELECT
        s.city_id,
        s.score,
        ROW_NUMBER() OVER (
            PARTITION BY s.city_id ORDER BY s.created_at DESC
        ) AS rn
    FROM
        public.summary AS s
    WHERE EXTRACT(YEAR FROM s.created_at) = $1
    )

    SELECT
        s.id
    FROM
        summary as s
    INNER JOIN latest_scores AS ls
      ON ls.city_id = s.city_id
    WHERE
        ls.rn = 1
    ORDER BY
        ls.score DESC
    LIMIT $2;
    "#;

    // Fetch them.
    let summary_ids: Vec<SummaryID> = SummaryID::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        query,
        vec![year.into(), count.into()],
    ))
    .all(db)
    .await?;

    // Return None if none were found.
    if summary_ids.is_empty() {
        return Ok(None);
    }

    // Otherwise return the full summary and the associated city.
    let mut cities_with_summaries: Vec<(city::Model, summary::Model)> =
        Vec::with_capacity(summary_ids.len());
    for summary_id in summary_ids {
        let (summary, opt_city) = Summary::find_by_id(summary_id.id)
            .find_also_related(entity::City)
            .one(db)
            .await?
            .expect("the summary must exist at this point");
        let city = opt_city.expect("a summary must belong to a city");
        cities_with_summaries.push((city, summary));
    }
    Ok(Some(cities_with_summaries))
}
