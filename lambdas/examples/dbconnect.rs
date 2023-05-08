use entity::bna;
use sea_orm::{Database, DbErr, EntityTrait, PaginatorTrait};
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = Database::connect(db_url).await?;
    let models = bna::Entity::find().paginate(&db, 10).fetch_page(0).await?;
    dbg!(models);

    Ok(())
}
