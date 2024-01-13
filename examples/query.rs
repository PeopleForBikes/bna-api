/// This example queries all the cities in the database and displays some
/// information about each of them.
use color_eyre::{eyre::Report, Result};
use dotenv::dotenv;
use entity::{prelude::*, sea_orm_active_enums::ApprovalStatus};
use once_cell::sync::OnceCell;
use sea_orm::{
    prelude::Uuid, ActiveValue, Database, DatabaseConnection, DbBackend, EntityTrait,
    FromQueryResult, QueryTrait,
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
    // let database_url = dotenv::var("DATABASE_URL")?;
    // let db = Database::connect(database_url).await?;
    // DATABASE_CONNECTION.set(db).unwrap();
    // let db = DATABASE_CONNECTION.get().unwrap();
    // let bna_uuid = Uuid::parse_str("18ca9450-2bfa-43a0-b8f3-2cd16952eba1").unwrap();
    // let summ: Option<BnaReport> = summary::Entity::find_by_id(bna_uuid)
    //     .column(summary::Column::BnaUuid)
    //     .column(summary::Column::CityId)
    //     .column_as(core_services::Column::Score, "cs_score")
    //     .column_as(recreation::Column::Score, "rec_score")
    //     .join(
    //         sea_orm::JoinType::InnerJoin,
    //         summary::Relation::CoreServices.def(),
    //     )
    //     .join(
    //         sea_orm::JoinType::InnerJoin,
    //         summary::Relation::Infrastructure.def(),
    //     )
    //     .join(
    //         sea_orm::JoinType::InnerJoin,
    //         summary::Relation::Recreation.def(),
    //     )
    //     .join(
    //         sea_orm::JoinType::InnerJoin,
    //         summary::Relation::Opportunity.def(),
    //     )
    //     .join(
    //         sea_orm::JoinType::InnerJoin,
    //         summary::Relation::Features.def(),
    //     )
    //     .build(DbBackend::Postgres)
    //     .to_string();
    // .into_model::<BnaReport>()
    // .one(db)
    // .await?;
    // dbg!(summ);

    let submission_model = entity::submission::ActiveModel {
        id: ActiveValue::NotSet,
        first_name: ActiveValue::Set("Floyd".to_string()),
        last_name: ActiveValue::Set("Martin".to_string()),
        title: ActiveValue::Set(None),
        organization: ActiveValue::Set(Some("organization".to_string())),
        email: ActiveValue::Set("floyd.martin@organization.com".to_string()),
        country: ActiveValue::Set("usa".to_string()),
        city: ActiveValue::Set("Provincetown".to_string()),
        region: ActiveValue::Set(Some("Massachussets".to_string())),
        fips_code: ActiveValue::Set("1234567".to_string()),
        consent: ActiveValue::Set(true),
        status: ActiveValue::NotSet,
    };
    let res = Submission::insert(submission_model)
        .build(DbBackend::Postgres)
        .to_string();
    dbg!(res);

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
