use entity::us_state;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};

pub(crate) async fn fetch_us_state(
    db: &DatabaseConnection,
    name: &str,
) -> Result<Option<us_state::Model>, DbErr> {
    us_state::Entity::find_by_id(name.to_string()).one(db).await
}

pub(crate) async fn fetch_us_states(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<us_state::Model>), DbErr> {
    let select = us_state::Entity::find();
    let count = select.clone().count(db).await?;
    let models = select
        .paginate(db, page_size)
        .fetch_page(page.saturating_sub(1))
        .await?;
    Ok((count, models))
}
