use super::db::{fetch_rating, fetch_ratings, fetch_ratings_city, fetch_ratings_summaries, Bna};
use crate::{database_connect, Context, ExecutionError};
use entity::{
    core_services, infrastructure, opportunity, people, recreation, retail, summary, transit,
    wrappers::bna::BNAPost,
};
use sea_orm::{ActiveModelTrait, ActiveValue};
use tracing::info;
use uuid::Uuid;

pub async fn get_ratings_summaries_adaptor(
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<summary::Model>), ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    Ok(fetch_ratings_summaries(&db, page, page_size).await?)
}

pub(crate) async fn get_rating_adaptor(
    rating_id: Uuid,
    ctx: Context,
) -> Result<Bna, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model: Option<super::db::Bna> = fetch_rating(&db, rating_id).await?;
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub(crate) async fn get_ratings_adaptor(
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<Bna>), ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    Ok(fetch_ratings(&db, page, page_size).await?)
}

pub(crate) async fn get_ratings_city_adaptor(
    rating_id: Uuid,
    ctx: Context,
) -> Result<(Bna, entity::city::Model), ExecutionError> {
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_city(&db, rating_id).await?;
    match model {
        Some((bna, city)) => Ok((bna, city)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub(crate) async fn post_ratings_adaptor(bna: BNAPost) -> Result<Bna, ExecutionError> {
    // Turn the model wrapper into active models.
    let summary = summary::ActiveModel {
        id: ActiveValue::NotSet,
        city_id: ActiveValue::Set(bna.summary.city_id),
        created_at: ActiveValue::NotSet,
        score: ActiveValue::NotSet,
        version: ActiveValue::Set(bna.summary.version),
    };
    info!("{:?}", summary);
    let core_services = core_services::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        dentists: ActiveValue::Set(bna.core_services.dentists),
        doctors: ActiveValue::Set(bna.core_services.doctors),
        grocery: ActiveValue::Set(bna.core_services.grocery),
        hospitals: ActiveValue::Set(bna.core_services.hospitals),
        pharmacies: ActiveValue::Set(bna.core_services.pharmacies),
        score: ActiveValue::Set(bna.core_services.score),
        social_services: ActiveValue::Set(bna.core_services.social_services),
    };
    info!("{:?}", core_services);
    let people = people::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        score: ActiveValue::Set(bna.people.score),
    };
    info!("{:?}", people);
    let retail = retail::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        score: ActiveValue::Set(bna.retail.score),
    };
    info!("{:?}", people);
    let transit = transit::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        score: ActiveValue::Set(bna.transit.score),
    };
    info!("{:?}", people);
    let infrastructure = infrastructure::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        low_stress_miles: ActiveValue::Set(bna.infrastructure.low_stress_miles),
        high_stress_miles: ActiveValue::Set(bna.infrastructure.high_stress_miles),
    };
    info!("{:?}", infrastructure);
    let opportunity = opportunity::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        employment: ActiveValue::Set(bna.opportunity.employment),
        higher_education: ActiveValue::Set(bna.opportunity.higher_education),
        k12_education: ActiveValue::Set(bna.opportunity.k12_education),
        score: ActiveValue::Set(bna.opportunity.score),
        technical_vocational_college: ActiveValue::Set(
            bna.opportunity.technical_vocational_college,
        ),
    };
    info!("{:?}", opportunity);
    let recreation = recreation::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        community_centers: ActiveValue::Set(bna.recreation.community_centers),
        parks: ActiveValue::Set(bna.recreation.parks),
        recreation_trails: ActiveValue::Set(bna.recreation.recreation_trails),
        score: ActiveValue::Set(bna.recreation.score),
    };
    info!("{:?}", recreation);

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;
    info!("DB acquired");

    // And insert a new entry for each model.
    let summary_model = summary.insert(&db).await?;
    let core_services_model = core_services.insert(&db).await?;
    let infrastructure_model = infrastructure.insert(&db).await?;
    let opportunity_model = opportunity.insert(&db).await?;
    let people_model = people.insert(&db).await?;
    let recreation_model = recreation.insert(&db).await?;
    let retail_model = retail.insert(&db).await?;
    let transit_model = transit.insert(&db).await?;
    let bna = Bna {
        id: summary_model.id,
        city_id: summary_model.city_id,
        score: summary_model.score,
        version: summary_model.version,
        low_stress_miles: infrastructure_model.low_stress_miles,
        high_stress_miles: infrastructure_model.high_stress_miles,
        community_centers: recreation_model.community_centers,
        parks: recreation_model.parks,
        recreation_trails: recreation_model.recreation_trails,
        recreation_score: recreation_model.score,
        employment: opportunity_model.employment,
        higher_education: opportunity_model.higher_education,
        k12_education: opportunity_model.k12_education,
        opportunity_score: opportunity_model.score,
        technical_vocational_college: opportunity_model.technical_vocational_college,
        dentists: core_services_model.dentists,
        doctors: core_services_model.doctors,
        grocery: core_services_model.grocery,
        hospitals: core_services_model.hospitals,
        pharmacies: core_services_model.pharmacies,
        coreservices_score: core_services_model.score,
        social_services: core_services_model.social_services,
        people: people_model.score,
        retail: retail_model.score,
        transit: transit_model.score,
    };
    info!("{:?}", bna);
    Ok(bna)
}
