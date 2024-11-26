use entity::{
    bna_pipeline, city, core_services, infrastructure, opportunity, people, recreation, retail,
    summary, transit,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, PaginatorTrait, QuerySelect,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::BNAComponent;

#[derive(Debug, FromQueryResult, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bna {
    // BNA Summary
    id: Uuid,
    city_id: Uuid,
    score: f64,
    version: String,

    // BNAInfrastructure
    low_stress_miles: Option<f64>,
    high_stress_miles: Option<f64>,

    // BNA Recreation
    community_centers: Option<f64>,
    parks: Option<f64>,
    recreation_trails: Option<f64>,
    recreation_score: Option<f64>,

    // BNA Opportunity
    employment: Option<f64>,
    higher_education: Option<f64>,
    k12_education: Option<f64>,
    opportunity_score: Option<f64>,
    technical_vocational_college: Option<f64>,

    // BNA Core Services
    dentists: Option<f64>,
    doctors: Option<f64>,
    grocery: Option<f64>,
    hospitals: Option<f64>,
    pharmacies: Option<f64>,
    coreservices_score: Option<f64>,
    social_services: Option<f64>,

    // BNA People
    people: Option<f64>,

    // BNA Retail
    retail: Option<f64>,

    // BNA Transit
    transit: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Summary {
    id: Uuid,
    city_id: Uuid,
    score: f64,
    version: String,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Infrastructure {
    low_stress_miles: Option<f64>,
    high_stress_miles: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Recreation {
    community_centers: Option<f64>,
    parks: Option<f64>,
    recreation_trails: Option<f64>,
    recreation_score: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Opportunity {
    employment: Option<f64>,
    higher_education: Option<f64>,
    k12_education: Option<f64>,
    opportunity_score: Option<f64>,
    technical_vocational_college: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct CoreServices {
    dentists: Option<f64>,
    doctors: Option<f64>,
    grocery: Option<f64>,
    hospitals: Option<f64>,
    pharmacies: Option<f64>,
    coreservices_score: Option<f64>,
    social_services: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct People {
    people: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Retail {
    retail: Option<f64>,
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Transit {
    transit: Option<f64>,
}

#[derive(Debug)]
pub enum BNAComponentValue {
    All(Bna),
    Summary(Summary),
    Infrastructure(Summary, Option<Infrastructure>),
    Recreation(Summary, Option<Recreation>),
    Opportunity(Summary, Option<Opportunity>),
    CoreServices(Summary, Option<CoreServices>),
    People(Summary, Option<People>),
    Retail(Summary, Option<Retail>),
    Transit(Summary, Option<Transit>),
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

pub async fn fetch_ratings_summary_with_parts(
    db: &DatabaseConnection,
    rating_id: Uuid,
    component: Option<BNAComponent>,
) -> Result<Option<BNAComponentValue>, sea_orm::DbErr> {
    let select = summary::Entity::find_by_id(rating_id);
    let component = component.unwrap_or(BNAComponent::All);
    dbg!(&component);
    let res = match component {
        BNAComponent::All => select
            .clone()
            .columns([
                entity::core_services::Column::Dentists,
                entity::core_services::Column::Doctors,
                entity::core_services::Column::Grocery,
                entity::core_services::Column::Hospitals,
                entity::core_services::Column::Pharmacies,
                entity::core_services::Column::SocialServices,
            ])
            .column_as(entity::core_services::Column::Score, "coreservices_score")
            .columns([
                entity::infrastructure::Column::HighStressMiles,
                entity::infrastructure::Column::LowStressMiles,
            ])
            .columns([
                entity::recreation::Column::CommunityCenters,
                entity::recreation::Column::Parks,
                entity::recreation::Column::RecreationTrails,
            ])
            .column_as(entity::recreation::Column::Score, "recreation_score")
            .columns([
                entity::opportunity::Column::Employment,
                entity::opportunity::Column::HigherEducation,
                entity::opportunity::Column::K12Education,
                entity::opportunity::Column::TechnicalVocationalCollege,
            ])
            .column_as(entity::opportunity::Column::Score, "opportunity_score")
            .column_as(entity::people::Column::Score, "people_score")
            .column_as(entity::retail::Column::Score, "retail_score")
            .column_as(entity::transit::Column::Score, "transit_score")
            .join(
                JoinType::InnerJoin,
                entity::summary::Relation::CoreServices.def(),
            )
            .join(
                JoinType::InnerJoin,
                entity::summary::Relation::Infrastructure.def(),
            )
            .join(
                JoinType::InnerJoin,
                entity::summary::Relation::Recreation.def(),
            )
            .join(
                JoinType::InnerJoin,
                entity::summary::Relation::Opportunity.def(),
            )
            .join(
                sea_orm::JoinType::InnerJoin,
                entity::summary::Relation::People.def(),
            )
            .join(
                sea_orm::JoinType::InnerJoin,
                entity::summary::Relation::Retail.def(),
            )
            .join(
                sea_orm::JoinType::InnerJoin,
                entity::summary::Relation::Transit.def(),
            )
            .into_model::<Bna>()
            .one(db)
            .await?
            .map(BNAComponentValue::All),
        BNAComponent::Summary => select
            .clone()
            .into_model::<Summary>()
            .one(db)
            .await?
            .map(BNAComponentValue::Summary),
        BNAComponent::Infratructure => select
            .clone()
            .find_also_related(infrastructure::Entity)
            .into_model::<Summary, Infrastructure>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::Infrastructure(m.0, m.1)),
        BNAComponent::Recreation => select
            .clone()
            .find_also_related(recreation::Entity)
            .into_model::<Summary, Recreation>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::Recreation(m.0, m.1)),
        BNAComponent::Opportunity => select
            .clone()
            .find_also_related(opportunity::Entity)
            .into_model::<Summary, Opportunity>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::Opportunity(m.0, m.1)),
        BNAComponent::CoreServices => select
            .clone()
            .find_also_related(core_services::Entity)
            .into_model::<Summary, CoreServices>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::CoreServices(m.0, m.1)),
        BNAComponent::People => select
            .clone()
            .find_also_related(people::Entity)
            .into_model::<Summary, People>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::People(m.0, m.1)),
        BNAComponent::Retail => select
            .clone()
            .find_also_related(retail::Entity)
            .into_model::<Summary, Retail>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::Retail(m.0, m.1)),
        BNAComponent::Transit => select
            .clone()
            .find_also_related(transit::Entity)
            .into_model::<Summary, Transit>()
            .one(db)
            .await?
            .map(|m| BNAComponentValue::Transit(m.0, m.1)),
    };
    Ok(res)
}

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
