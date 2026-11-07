use entity::bna_pipeline;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use uuid::Uuid;

pub async fn fetch_bna_pipeline(
    db: &DatabaseConnection,
    pipeline_id: Uuid,
) -> Result<Option<bna_pipeline::Model>, sea_orm::DbErr> {
    bna_pipeline::Entity::find_by_id(pipeline_id).one(db).await
}

pub async fn fetch_bna_pipelines(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<bna_pipeline::Model>), sea_orm::DbErr> {
    let select = bna_pipeline::Entity::find().order_by_asc(bna_pipeline::Column::StateMachineId);
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page.saturating_sub(1))
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}
