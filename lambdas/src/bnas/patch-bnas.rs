use dotenv::dotenv;
use effortless::{
    api::{
        entry_not_found, invalid_path_parameter, missing_parameter, parse_path_parameter,
        parse_request_body,
    },
    fragment::BnaRequestExt,
};
use entity::{
    core_services, features, infrastructure, opportunity, prelude, recreation, summary,
    wrappers::bna::BNAPatch,
};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{prelude::Uuid, ActiveValue, EntityTrait};
use serde_json::json;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await.map_err(|e| {
        info!("{e}");
        e
    })
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve the ID of the entry to update.
    let parameter = "id";
    let id = event
        .path_parameter::<Uuid>(parameter)
        .ok_or(missing_parameter(&event, parameter))?
        .map_err(|e| invalid_path_parameter(&event, parameter, e.to_string().as_str()))?;

    // Extract and deserialize the data.
    let wrapper = match parse_request_body::<BNAPatch>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Turn the model wrapper into active models.
    let summary = summary::ActiveModel {
        bna_uuid: ActiveValue::NotSet,
        city_id: wrapper
            .summary
            .city_id
            .map_or(ActiveValue::NotSet, ActiveValue::Set),
        created_at: ActiveValue::NotSet,
        score: ActiveValue::Set(wrapper.summary.score),
        version: ActiveValue::Set(wrapper.summary.version),
    };
    let core_services = core_services::ActiveModel {
        bna_uuid: ActiveValue::NotSet,
        dentists: ActiveValue::Set(wrapper.core_services.dentists),
        doctors: ActiveValue::Set(wrapper.core_services.doctors),
        grocery: ActiveValue::Set(wrapper.core_services.grocery),
        hospitals: ActiveValue::Set(wrapper.core_services.hospitals),
        pharmacies: ActiveValue::Set(wrapper.core_services.pharmacies),
        score: ActiveValue::Set(wrapper.core_services.score),
        social_services: ActiveValue::Set(wrapper.core_services.social_services),
    };
    let features = features::ActiveModel {
        bna_uuid: ActiveValue::NotSet,
        people: ActiveValue::Set(wrapper.features.people),
        retail: ActiveValue::Set(wrapper.features.retail),
        transit: ActiveValue::Set(wrapper.features.transit),
    };
    let infrastructure = infrastructure::ActiveModel {
        bna_uuid: ActiveValue::NotSet,
        low_stress_miles: ActiveValue::Set(wrapper.infrastructure.low_stress_miles),
        high_stress_miles: ActiveValue::Set(wrapper.infrastructure.high_stress_miles),
    };
    let opportunity = opportunity::ActiveModel {
        bna_uuid: ActiveValue::NotSet,
        employment: ActiveValue::Set(wrapper.opportunity.employment),
        higher_education: ActiveValue::Set(wrapper.opportunity.higher_education),
        k12_education: ActiveValue::Set(wrapper.opportunity.k12_education),
        score: ActiveValue::Set(wrapper.opportunity.score),
        technical_vocational_college: ActiveValue::Set(
            wrapper.opportunity.technical_vocational_college,
        ),
    };
    let recreation = recreation::ActiveModel {
        bna_uuid: ActiveValue::NotSet,
        community_centers: ActiveValue::Set(wrapper.recreation.community_centers),
        parks: ActiveValue::Set(wrapper.recreation.parks),
        recreation_trails: ActiveValue::Set(wrapper.recreation.recreation_trails),
        score: ActiveValue::Set(wrapper.recreation.score),
    };

    Ok(Body::Empty.into_response().await)
}
