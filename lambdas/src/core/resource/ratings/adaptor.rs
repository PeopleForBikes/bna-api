use super::{
    db::{fetch_rating, fetch_ratings, fetch_ratings_city, Bna},
    schema::RatingPost,
};
use crate::{Context, ExecutionError};
use entity::{
    city, core_services, infrastructure, measure, opportunity, people, recreation, retail, summary,
    transit,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use tracing::info;
use uuid::Uuid;

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
    ctx: Context,
) -> Result<Bna, ExecutionError> {
    // Write the rating in a single transaction, so that a failure leaves no
    // partial rating behind. Returning early drops the transaction, which rolls
    // it back.
    let txn = db.begin().await?;

    // Ensure the rated city exists.
    if city::Entity::find()
        .filter(city::Column::Id.eq(rating.city_id))
        .one(&txn)
        .await?
        .is_none()
    {
        return Err(ExecutionError::InvalidAttribute(
            ctx.request_id(),
            ctx.source(),
            format!("the city_id does not match any city: {}", rating.city_id),
        ));
    }

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
    info!("{:?}", retail);
    let transit = transit::ActiveModel {
        id: ActiveValue::Set(rating_id),
        score: rating
            .transit
            .transit
            .map_or(ActiveValue::NotSet, |v| ActiveValue::Set(Some(v))),
    };
    info!("{:?}", transit);
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
    let summary_model = summary.insert(&txn).await?;
    let core_services_model = core_services.insert(&txn).await?;
    let infrastructure_model = infrastructure.insert(&txn).await?;
    let opportunity_model = opportunity.insert(&txn).await?;
    let people_model = people.insert(&txn).await?;
    let recreation_model = recreation.insert(&txn).await?;
    let retail_model = retail.insert(&txn).await?;
    let transit_model = transit.insert(&txn).await?;
    let measure_model = measure.insert(&txn).await?;
    txn.commit().await?;
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
        people_score: people_model.score,
        retail_score: retail_model.score,
        transit_score: transit_model.score,
        buffered_lane: measure_model.buffered_lane,
        lane: measure_model.lane,
        path: measure_model.path,
        sharrow: measure_model.sharrow,
        track: measure_model.track,
    };
    info!("{:?}", bna);
    Ok(bna)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sea_orm::{DbBackend, DbErr, MockDatabase, Transaction};
    use serde_json::json;

    const CITY_ID: Uuid = Uuid::from_u128(1);

    fn context() -> Context {
        Context {
            request_id: None,
            source: "/ratings".to_string(),
        }
    }

    fn rating_post() -> RatingPost {
        serde_json::from_value(json!({
            "city_id": CITY_ID,
            "version": "25.10",
            "score": 50.0,
            "pop_size": 1,
            "population": 1000,
            "infrastructure": {},
            "recreation": {},
            "opportunity": {},
            "core_services": {},
            "people": {},
            "retail": {},
            "transit": {},
            "measure": {}
        }))
        .unwrap()
    }

    fn city() -> city::Model {
        city::Model {
            id: CITY_ID,
            country: "United States".to_string(),
            state: "Texas".to_string(),
            name: "Austin".to_string(),
            latitude: None,
            longitude: None,
            region: None,
            state_abbrev: None,
            residential_speed_limit: None,
            created_at: Utc::now().into(),
            updated_at: None,
            fips_code: None,
        }
    }

    fn summary() -> summary::Model {
        summary::Model {
            id: Uuid::from_u128(2),
            city_id: CITY_ID,
            created_at: Utc::now().into(),
            pop_size: 1,
            population: 1000,
            residential_speed_limit_override: None,
            score: 50.0,
            version: "25.10".to_string(),
        }
    }

    fn core_services() -> core_services::Model {
        core_services::Model {
            id: Uuid::from_u128(2),
            dentists: None,
            doctors: None,
            grocery: None,
            hospitals: None,
            pharmacies: None,
            score: None,
            social_services: None,
        }
    }

    /// Returns the SQL of the statements run, in order.
    fn statements(log: Vec<Transaction>) -> Vec<String> {
        log.iter()
            .flat_map(|t| t.statements().iter().map(|s| s.sql.clone()))
            .collect()
    }

    #[tokio::test]
    async fn test_post_rating_rolls_back_when_an_insert_fails() {
        // The city exists, the first two inserts succeed, the third one fails.
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[city()]])
            .append_query_results([[summary()]])
            .append_query_results([[core_services()]])
            .append_query_errors([DbErr::Custom("connection lost".to_string())])
            .into_connection();

        let result = post_ratings_adaptor(&db, rating_post(), context()).await;

        assert!(result.is_err());
        let sql = statements(db.into_transaction_log());
        assert_eq!(sql.first().map(String::as_str), Some("BEGIN"));
        assert_eq!(sql.last().map(String::as_str), Some("ROLLBACK"));
        assert!(!sql.iter().any(|s| s == "COMMIT"));
    }
}
