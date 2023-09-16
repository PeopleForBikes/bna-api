/// This example queries all the cities in the database and displays some
/// information about each of them.
use color_eyre::{eyre::Report, Result};
use dotenv::dotenv;
use entity::recreation;
use entity::{city, core_services, summary};
use migration::Alias;
use once_cell::sync::OnceCell;
use sea_orm::sea_query::Expr;
use sea_orm::DbBackend;
use sea_orm::FromQueryResult;
use sea_orm::QueryTrait;
use sea_orm::{
    prelude::Uuid, Database, DatabaseConnection, EntityTrait, QuerySelect, RelationTrait,
};

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

#[derive(Debug, FromQueryResult)]
struct BnaReport {
    bna_uuid: Uuid,
    city_id: Uuid,
    score: f64,
    cs_score: f64,
    rec_score: f64,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    // Set the database connection.
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();
    let db = DATABASE_CONNECTION.get().unwrap();
    let bna_uuid = Uuid::parse_str("18ca9450-2bfa-43a0-b8f3-2cd16952eba1").unwrap();
    let summ: Option<BnaReport> = summary::Entity::find_by_id(bna_uuid)
        .column(summary::Column::BnaUuid)
        .column(summary::Column::CityId)
        .column_as(core_services::Column::Score, "cs_score")
        .column_as(recreation::Column::Score, "rec_score")
        .join(
            sea_orm::JoinType::InnerJoin,
            summary::Relation::CoreServices.def(),
        )
        .join(
            sea_orm::JoinType::InnerJoin,
            summary::Relation::Infrastructure.def(),
        )
        .join(
            sea_orm::JoinType::InnerJoin,
            summary::Relation::Recreation.def(),
        )
        .join(
            sea_orm::JoinType::InnerJoin,
            summary::Relation::Opportunity.def(),
        )
        .join(
            sea_orm::JoinType::InnerJoin,
            summary::Relation::Features.def(),
        )
        // .build(DbBackend::Postgres)
        // .to_string();
        .into_model::<BnaReport>()
        .one(db)
        .await?;

    dbg!(summ);
    Ok(())
}

// SELECT SUMMARY.SCORE,
// 	CORE_SERVICES.SCORE AS CSSCORE,
// 	INFRASTRUCTURE.low_Stress_miles,
// FROM SUMMARY
// JOIN CORE_SERVICES ON CORE_SERVICES.BNA_UUID = SUMMARY.BNA_UUID
// JOIN INFRASTRUCTURE ON INFRASTRUCTURE.BNA_UUID = SUMMARY.BNA_UUID
// JOIN RECREATION ON RECREATION.BNA_UUID = SUMMARY.BNA_UUID
// JOIN OPPORTUNITY ON OPPORTUNITY.BNA_UUID = SUMMARY.BNA_UUID
// JOIN FEATURES ON FEATURES.BNA_UUID = SUMMARY.BNA_UUID
// WHERE SUMMARY.BNA_UUID = '18ca9450-2bfa-43a0-b8f3-2cd16952eba1' ;
