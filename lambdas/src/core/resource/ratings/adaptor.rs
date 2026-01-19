use super::{
    db::{fetch_rating, fetch_ratings, fetch_ratings_city, fetch_ratings_summaries, Bna},
    schema::RatingPost,
};
use crate::{Context, ExecutionError};
use entity::{
    core_services, infrastructure, measure, opportunity, people, recreation, retail, summary,
    transit,
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use tracing::info;
use uuid::Uuid;

pub async fn get_ratings_summaries_adaptor(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<summary::Model>), ExecutionError> {
    // Fetch a page of summary.
    Ok(fetch_ratings_summaries(db, page, page_size).await?)
}

pub(crate) async fn get_rating_adaptor(
    db: &DatabaseConnection,
    rating_id: Uuid,
    ctx: Context,
) -> Result<Bna, ExecutionError> {
    // Fetch the model.
    let model: Option<super::db::Bna> = fetch_rating(db, rating_id).await?;
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
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<Bna>), ExecutionError> {
    // Fetch a page of summary.
    Ok(fetch_ratings(db, page, page_size).await?)
}

pub(crate) async fn get_ratings_city_adaptor(
    db: &DatabaseConnection,
    rating_id: Uuid,
    ctx: Context,
) -> Result<(Bna, entity::city::Model), ExecutionError> {
    // Fetch the model.
    let model = fetch_ratings_city(db, rating_id).await?;
    match model {
        Some((bna, city)) => Ok((bna, city)),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a rating with the ID {rating_id}"),
        )),
    }
}

pub(crate) async fn post_ratings_adaptor(
    db: &DatabaseConnection,
    rating: RatingPost,
) -> Result<Bna, ExecutionError> {
    // Generate a rating id.
    let rating_id = Uuid::new_v4();

    // Turn the model wrapper into active models.
    let summary = summary::ActiveModel {
        id: ActiveValue::Set(rating_id),
        city_id: ActiveValue::Set(rating.city_id),
        created_at: ActiveValue::NotSet,
        score: ActiveValue::Set(rating.score),
        version: ActiveValue::Set(rating.version),
        pop_size: ActiveValue::Set(rating.pop_size),
        population: ActiveValue::Set(rating.population),
        residential_speed_limit_override: rating
            .speed_limit_override
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
    };
    info!("{:?}", summary);
    let core_services = core_services::ActiveModel {
        id: ActiveValue::Set(rating_id),
        dentists: ActiveValue::Set(rating.core_services.dentists),
        doctors: ActiveValue::Set(rating.core_services.doctors),
        grocery: ActiveValue::Set(rating.core_services.grocery),
        hospitals: ActiveValue::Set(rating.core_services.hospitals),
        pharmacies: ActiveValue::Set(rating.core_services.pharmacies),
        score: ActiveValue::Set(rating.core_services.score),
        social_services: ActiveValue::Set(rating.core_services.social_services),
    };
    info!("{:?}", core_services);
    let people = people::ActiveModel {
        id: ActiveValue::Set(rating_id),
        score: rating
            .people
            .people
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
    };
    info!("{:?}", people);
    let retail = retail::ActiveModel {
        id: ActiveValue::Set(rating_id),
        score: rating
            .retail
            .retail
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
    };
    info!("{:?}", people);
    let transit = transit::ActiveModel {
        id: ActiveValue::Set(rating_id),
        score: rating
            .transit
            .transit
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
    };
    info!("{:?}", people);
    let infrastructure = infrastructure::ActiveModel {
        id: ActiveValue::Set(rating_id),
        low_stress_miles: ActiveValue::Set(rating.infrastructure.low_stress_miles),
        high_stress_miles: ActiveValue::Set(rating.infrastructure.high_stress_miles),
    };
    info!("{:?}", infrastructure);
    let opportunity = opportunity::ActiveModel {
        id: ActiveValue::Set(rating_id),
        employment: ActiveValue::Set(rating.opportunity.employment),
        higher_education: ActiveValue::Set(rating.opportunity.higher_education),
        k12_education: ActiveValue::Set(rating.opportunity.k12_education),
        score: ActiveValue::Set(rating.opportunity.score),
        technical_vocational_college: ActiveValue::Set(
            rating.opportunity.technical_vocational_college,
        ),
    };
    info!("{:?}", opportunity);
    let recreation = recreation::ActiveModel {
        id: ActiveValue::Set(rating_id),
        community_centers: ActiveValue::Set(rating.recreation.community_centers),
        parks: ActiveValue::Set(rating.recreation.parks),
        recreation_trails: rating
            .recreation
            .trails
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
        score: ActiveValue::Set(rating.recreation.score),
    };
    info!("{:?}", recreation);
    let measure = measure::ActiveModel {
        id: ActiveValue::Set(rating_id),
        buffered_lane: rating
            .measure
            .buffered_lane
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
        lane: rating
            .measure
            .lane
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
        path: rating
            .measure
            .path
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
        sharrow: rating
            .measure
            .sharrow
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
        track: rating
            .measure
            .track
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
    };
    info!("{:?}", measure);

    // And insert a new entry for each model.
    let summary_model = summary.insert(db).await?;
    let core_services_model = core_services.insert(db).await?;
    let infrastructure_model = infrastructure.insert(db).await?;
    let opportunity_model = opportunity.insert(db).await?;
    let people_model = people.insert(db).await?;
    let recreation_model = recreation.insert(db).await?;
    let retail_model = retail.insert(db).await?;
    let transit_model = transit.insert(db).await?;
    let measure_model = measure.insert(db).await?;
    let bna = Bna {
        id: summary_model.id,
        city_id: summary_model.city_id,
        pop_size: summary_model.pop_size,
        population: summary_model.population,
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
        buffered_lane: measure_model.buffered_lane,
        lane: measure_model.lane,
        path: measure_model.path,
        sharrow: measure_model.sharrow,
        track: measure_model.track,
    };
    info!("{:?}", bna);
    Ok(bna)
}
