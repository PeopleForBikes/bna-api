use super::{
    db::{
        fetch_ratings_analyses, fetch_ratings_analysis, fetch_ratings_city,
        fetch_ratings_summaries, fetch_ratings_summary_with_parts,
    },
    BNAComponent,
};
use crate::{database_connect, Context, ExecutionError, PageFlow, Paginatron};
use entity::{
    core_services, infrastructure, opportunity, people, recreation, retail, summary, transit,
    wrappers::{
        bna::{BNAPost, RatingFlat},
        bna_pipeline::{BNAPipelinePatch, BNAPipelinePost},
    },
};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serde_json::{json, Value};
use tracing::info;
use uuid::Uuid;

pub async fn get_ratings_summaries_adaptor(
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_ratings_summaries(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_rating_adaptor(
    rating_id: Uuid,
    component: Option<BNAComponent>,
    ctx: Context,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_summary_with_parts(&db, rating_id, component).await?;
    match model {
        Some(model) => match model {
            super::db::BNAComponentValue::All(bna) => Ok(json!(bna)),
            super::db::BNAComponentValue::Summary(summary) => Ok(json!(summary)),
            super::db::BNAComponentValue::Infrastructure(summary, infrastructure) => {
                Ok(json!((summary, infrastructure)))
            }
            super::db::BNAComponentValue::Recreation(summary, recreation) => {
                Ok(json!((summary, recreation)))
            }
            super::db::BNAComponentValue::Opportunity(summary, opportunity) => {
                Ok(json!((summary, opportunity)))
            }
            super::db::BNAComponentValue::CoreServices(summary, core_services) => {
                Ok(json!((summary, core_services)))
            }
            super::db::BNAComponentValue::People(summary, people) => Ok(json!((summary, people))),
            super::db::BNAComponentValue::Retail(summary, retail) => Ok(json!((summary, retail))),
            super::db::BNAComponentValue::Transit(summary, transit) => {
                Ok(json!((summary, transit)))
            }
        },
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub async fn get_ratings_adaptor(page: u64, page_size: u64) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_ratings_summaries(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_ratings_analyses_adaptor(
    page: u64,
    page_size: u64,
) -> Result<PageFlow, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch a page of summary.
    let (total_items, models) = fetch_ratings_analyses(&db, page, page_size).await?;

    // Return the paginated response.
    Ok(PageFlow::new(
        Paginatron::new(None, total_items, page, page_size),
        json!(models),
    ))
}

pub async fn get_ratings_analysis_adaptor(
    analysis_id: Uuid,
    ctx: Context,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_analysis(&db, analysis_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {analysis_id}"),
        )),
    }
}

pub async fn get_ratings_city_adaptor(
    rating_id: Uuid,
    ctx: Context,
) -> Result<Value, ExecutionError> {
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Fetch the model.
    let model = fetch_ratings_city(&db, rating_id).await?;
    match model {
        Some(model) => Ok(json!(model)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub async fn post_ratings_analysis_adaptor(
    bna_pipeline: BNAPipelinePost,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Turn the post model into an active model.
    let active_model = bna_pipeline.into_active_model();

    // And insert a new entry.
    info!(
        "inserting Brokenspoke pipeline into database: {:?}",
        active_model
    );
    let model = active_model.insert(&db).await?;
    Ok(json!(model))
}

pub async fn patch_ratings_analysis_adaptor(
    bna_pipeline: BNAPipelinePatch,
    analysis_id: Uuid,
) -> Result<Value, ExecutionError> {
    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Turn the patch model into an active model.
    let mut active_model = bna_pipeline.into_active_model();
    active_model.state_machine_id = ActiveValue::Unchanged(analysis_id);

    // Update the entry.
    let model = active_model.update(&db).await?;
    Ok(json!(model))
}

pub async fn post_ratings_adaptor(bna: BNAPost) -> Result<Value, ExecutionError> {
    // Turn the model wrapper into active models.
    let summary = summary::ActiveModel {
        id: ActiveValue::Set(bna.summary.rating_id),
        city_id: ActiveValue::Set(bna.summary.city_id),
        created_at: ActiveValue::NotSet,
        score: ActiveValue::Set(bna.summary.score),
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
    let rating = RatingFlat {
        core_services: core_services_model,
        infrastructure: infrastructure_model,
        opportunity: opportunity_model,
        people: people_model,
        recreation: recreation_model,
        retail: retail_model,
        summary: summary_model,
        transit: transit_model,
    };
    info!("{:?}", rating);
    Ok(json!(rating))
}
