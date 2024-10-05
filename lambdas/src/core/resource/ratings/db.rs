use entity::{bna_pipeline, city, summary};
use sea_orm::{DatabaseConnection, EntityTrait, FromQueryResult, PaginatorTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(FromQueryResult, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bna {
    // BNA Summary
    pub bna_id: Uuid,
    pub city_id: Uuid,
    pub score: f64,
    pub version: String,

    // BNAInfrastructure
    pub low_stress_miles: Option<f64>,
    pub high_stress_miles: Option<f64>,

    // BNA Recreation
    pub community_centers: Option<f64>,
    pub parks: Option<f64>,
    pub recreation_trails: Option<f64>,
    pub recreation_score: Option<f64>,

    // BNA Opportunity
    pub employment: Option<f64>,
    pub higher_education: Option<f64>,
    pub k12_education: Option<f64>,
    pub opportunity_score: Option<f64>,
    pub technical_vocational_college: Option<f64>,

    // BNA Core Services
    pub dentists: Option<f64>,
    pub doctors: Option<f64>,
    pub grocery: Option<f64>,
    pub hospitals: Option<f64>,
    pub pharmacies: Option<f64>,
    pub coreservices_score: Option<f64>,
    pub social_services: Option<f64>,

    // BNA People
    pub people: Option<f64>,

    // BNA Retail
    pub retail: Option<f64>,

    // BNA Transit
    pub transit: Option<f64>,
}

pub async fn fetch_ratings_summary(
    db: &DatabaseConnection,
    rating_id: Uuid,
) -> Result<Option<summary::Model>, sea_orm::DbErr> {
    summary::Entity::find_by_id(rating_id).one(db).await
}

pub async fn fetch_ratings_summaries(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<summary::Model>), sea_orm::DbErr> {
    let select = summary::Entity::find();
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}

// pub async fn fetch_ratings_summary(
//     db: &DatabaseConnection,
//     bna_id: Uuid,
//     component: Option<BNAComponent>,
// ) -> Result<Option<BNA>, sea_orm::DbErr> {
//     let select = summary::Entity::find_by_id(bna_id);
//     let component = component.unwrap_or(BNAComponent::All);
//     let res = match component {
//         BNAComponent::All => {
//             select
//                 .clone()
//                 .columns([
//                     entity::core_services::Column::Dentists,
//                     entity::core_services::Column::Doctors,
//                     entity::core_services::Column::Grocery,
//                     entity::core_services::Column::Hospitals,
//                     entity::core_services::Column::Pharmacies,
//                     entity::core_services::Column::SocialServices,
//                 ])
//                 .column_as(entity::core_services::Column::Score, "coreservices_score")
//                 .columns([
//                     entity::infrastructure::Column::HighStressMiles,
//                     entity::infrastructure::Column::LowStressMiles,
//                 ])
//                 .columns([
//                     entity::recreation::Column::CommunityCenters,
//                     entity::recreation::Column::Parks,
//                     entity::recreation::Column::RecreationTrails,
//                 ])
//                 .column_as(entity::recreation::Column::Score, "recreation_score")
//                 .columns([
//                     entity::opportunity::Column::Employment,
//                     entity::opportunity::Column::HigherEducation,
//                     entity::opportunity::Column::K12Education,
//                     entity::opportunity::Column::TechnicalVocationalCollege,
//                 ])
//                 .column_as(entity::opportunity::Column::Score, "opportunity_score")
//                 .column_as(entity::people::Column::Score, "people_score")
//                 .column_as(entity::retail::Column::Score, "retail_score")
//                 .column_as(entity::transit::Column::Score, "transit_score")
//                 .join(
//                     JoinType::InnerJoin,
//                     entity::summary::Relation::CoreServices.def(),
//                 )
//                 .join(
//                     JoinType::InnerJoin,
//                     entity::summary::Relation::Infrastructure.def(),
//                 )
//                 .join(
//                     JoinType::InnerJoin,
//                     entity::summary::Relation::Recreation.def(),
//                 )
//                 .join(
//                     JoinType::InnerJoin,
//                     entity::summary::Relation::Opportunity.def(),
//                 )
//                 .join(
//                     sea_orm::JoinType::InnerJoin,
//                     entity::summary::Relation::People.def(),
//                 )
//                 .join(
//                     sea_orm::JoinType::InnerJoin,
//                     entity::summary::Relation::Retail.def(),
//                 )
//                 .join(
//                     sea_orm::JoinType::InnerJoin,
//                     entity::summary::Relation::Transit.def(),
//                 )
//                 .into_model::<BNA>()
//                 .one(db)
//                 .await?
//         }
//         BNAComponent::Summary => {
//             let model: Option<summary::Model> = select.clone().one(&db).await?;
//         }
//         BNAComponent::Infratructure => {
//             let model:Option<(summary::Model, Option<infrastructure::Model>)>  = select
//                 .clone()
//                 .find_also_related(infrastructure::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//         BNAComponent::Recreation => {
//             let model = select
//                 .clone()
//                 .find_also_related(recreation::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//         BNAComponent::Opportunity => {
//             let model = select
//                 .clone()
//                 .find_also_related(opportunity::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//         BNAComponent::CoreServices => {
//             let model = select
//                 .clone()
//                 .find_also_related(core_services::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//         BNAComponent::People => {
//             let model = select
//                 .clone()
//                 .find_also_related(people::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//         BNAComponent::Retail => {
//             let model = select
//                 .clone()
//                 .find_also_related(retail::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//         BNAComponent::Transit => {
//             let model = select
//                 .clone()
//                 .find_also_related(transit::Entity)
//                 .one(&db)
//                 .await?;
//             match model {
//                 Some(model) => json!(model).into_response().await,
//                 None => entry_not_found(&event).into(),
//             }
//         }
//     };
//     Ok(res)
// }

pub async fn fetch_ratings_analyses(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<bna_pipeline::Model>), sea_orm::DbErr> {
    let select = bna_pipeline::Entity::find();
    let models = select
        .clone()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}

pub async fn fetch_ratings_analysis(
    db: &DatabaseConnection,
    analysis_id: Uuid,
) -> Result<Option<bna_pipeline::Model>, sea_orm::DbErr> {
    bna_pipeline::Entity::find_by_id(analysis_id).one(db).await
}

pub async fn fetch_ratings_city(
    db: &DatabaseConnection,
    rating_id: Uuid,
) -> Result<Option<(summary::Model, Option<city::Model>)>, sea_orm::DbErr> {
    summary::Entity::find_by_id(rating_id)
        .find_also_related(city::Entity)
        .one(db)
        .await
}
