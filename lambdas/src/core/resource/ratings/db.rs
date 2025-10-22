use entity::{city, summary};
use sea_orm::{
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, PaginatorTrait, QuerySelect,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromQueryResult, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Bna {
    // BNA Summary
    pub(crate) id: Uuid,
    pub(crate) city_id: Uuid,
    pub(crate) score: f64,
    pub(crate) pop_size: i32,
    pub(crate) population: i32,
    pub(crate) version: String,

    // BNAInfrastructure
    pub(crate) low_stress_miles: Option<f64>,
    pub(crate) high_stress_miles: Option<f64>,

    // BNA Recreation
    pub(crate) community_centers: Option<f64>,
    pub(crate) parks: Option<f64>,
    pub(crate) recreation_trails: Option<f64>,
    pub(crate) recreation_score: Option<f64>,

    // BNA Opportunity
    pub(crate) employment: Option<f64>,
    pub(crate) higher_education: Option<f64>,
    pub(crate) k12_education: Option<f64>,
    pub(crate) opportunity_score: Option<f64>,
    pub(crate) technical_vocational_college: Option<f64>,

    // BNA Core Services
    pub(crate) dentists: Option<f64>,
    pub(crate) doctors: Option<f64>,
    pub(crate) grocery: Option<f64>,
    pub(crate) hospitals: Option<f64>,
    pub(crate) pharmacies: Option<f64>,
    pub(crate) coreservices_score: Option<f64>,
    pub(crate) social_services: Option<f64>,

    // BNA People
    pub(crate) people: Option<f64>,

    // BNA Retail
    pub(crate) retail: Option<f64>,

    // BNA Transit
    pub(crate) transit: Option<f64>,

    // Measure
    pub(crate) buffered_lane: Option<f64>,
    pub(crate) lane: Option<f64>,
    pub(crate) path: Option<f64>,
    pub(crate) sharrow: Option<f64>,
    pub(crate) track: Option<f64>,
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

pub async fn fetch_ratings_city(
    db: &DatabaseConnection,
    rating_id: Uuid,
) -> Result<Option<(Bna, city::Model)>, sea_orm::DbErr> {
    match summary::Entity::find_by_id(rating_id)
        .find_also_related(city::Entity)
        .one(db)
        .await?
    {
        Some((_, city)) => {
            let bna = fetch_rating(db, rating_id).await?;
            Ok(Some((
                bna.expect("we already found this bna so it must exist"),
                city.expect("a city must be attached to a rating"),
            )))
        }
        None => Ok(None),
    }
}

pub async fn fetch_rating(
    db: &DatabaseConnection,
    rating_id: Uuid,
) -> Result<Option<Bna>, sea_orm::DbErr> {
    let res = summary::Entity::find_by_id(rating_id)
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
        .await?;
    Ok(res)
}

pub async fn fetch_ratings(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<Bna>), sea_orm::DbErr> {
    let select = summary::Entity::find()
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
        );
    let models = select
        .clone()
        .into_model::<Bna>()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;
    let count = select.count(db).await?;
    Ok((count, models))
}
