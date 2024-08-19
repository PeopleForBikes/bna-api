use dotenv::dotenv;
use effortless::{api::parse_request_body, response::make_json_created_response};
use entity::{
    core_services, infrastructure, opportunity, people, recreation, retail, summary, transit,
    wrappers::bna::BNAPost,
};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambdas::database_connect;
use sea_orm::{ActiveModelTrait, ActiveValue};
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

    // Extract and serialize the data.
    info!("Parsing body into BNAPost...");
    let wrapper = match parse_request_body::<BNAPost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Note(rgreinho): We are note supposed to do that. The Brokenspoke-analyzer
    // already computes the scores for us.
    // Refresh the scores.
    // wrapper.core_services.refresh_score();
    // wrapper.opportunity.refresh_score();
    // wrapper.recreation.refresh_score();
    // wrapper.refresh_score();

    // Turn the model wrapper into active models.
    let summary = summary::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        city_id: ActiveValue::Set(wrapper.summary.city_id),
        created_at: ActiveValue::NotSet,
        score: ActiveValue::Set(wrapper.summary.score),
        version: ActiveValue::Set(wrapper.summary.version),
    };
    info!("{:?}", summary);
    let core_services = core_services::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        dentists: ActiveValue::Set(wrapper.core_services.dentists),
        doctors: ActiveValue::Set(wrapper.core_services.doctors),
        grocery: ActiveValue::Set(wrapper.core_services.grocery),
        hospitals: ActiveValue::Set(wrapper.core_services.hospitals),
        pharmacies: ActiveValue::Set(wrapper.core_services.pharmacies),
        score: ActiveValue::Set(wrapper.core_services.score),
        social_services: ActiveValue::Set(wrapper.core_services.social_services),
    };
    info!("{:?}", core_services);
    let people = people::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        score: ActiveValue::Set(wrapper.people.score),
    };
    info!("{:?}", people);
    let retail = retail::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        score: ActiveValue::Set(wrapper.retail.score),
    };
    info!("{:?}", people);
    let transit = transit::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        score: ActiveValue::Set(wrapper.transit.score),
    };
    info!("{:?}", people);
    let infrastructure = infrastructure::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        low_stress_miles: ActiveValue::Set(wrapper.infrastructure.low_stress_miles),
        high_stress_miles: ActiveValue::Set(wrapper.infrastructure.high_stress_miles),
    };
    info!("{:?}", infrastructure);
    let opportunity = opportunity::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        employment: ActiveValue::Set(wrapper.opportunity.employment),
        higher_education: ActiveValue::Set(wrapper.opportunity.higher_education),
        k12_education: ActiveValue::Set(wrapper.opportunity.k12_education),
        score: ActiveValue::Set(wrapper.opportunity.score),
        technical_vocational_college: ActiveValue::Set(
            wrapper.opportunity.technical_vocational_college,
        ),
    };
    info!("{:?}", opportunity);
    let recreation = recreation::ActiveModel {
        bna_id: ActiveValue::Set(wrapper.summary.bna_uuid),
        community_centers: ActiveValue::Set(wrapper.recreation.community_centers),
        parks: ActiveValue::Set(wrapper.recreation.parks),
        recreation_trails: ActiveValue::Set(wrapper.recreation.recreation_trails),
        score: ActiveValue::Set(wrapper.recreation.score),
    };
    info!("{:?}", recreation);

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // And insert a new entry for each model.
    let summary_res = summary.insert(&db).await?;
    let core_services_res = core_services.insert(&db).await?;
    let people_res = people.insert(&db).await?;
    let retail_res = retail.insert(&db).await?;
    let transit_res = transit.insert(&db).await?;
    let infrastructure_res = infrastructure.insert(&db).await?;
    let opportunity_res = opportunity.insert(&db).await?;
    let recreation_res = recreation.insert(&db).await?;
    let res = (
        summary_res,
        core_services_res,
        people_res,
        retail_res,
        transit_res,
        infrastructure_res,
        opportunity_res,
        recreation_res,
    );
    info!("{:?}", res);
    let response =
        make_json_created_response(json!(res).to_string()).expect("unable to build http::Response");
    Ok(response)

    // Ok(Body::Empty.into_response().await)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use entity::wrappers::bna::{
    //     BNACoreServices, BNAFeatures, BNAInfrastructure, BNAOpportunity, BNARecreation, BNASummary,
    // };
    // use lambda_http::{http, RequestExt};
    // use sea_orm::prelude::Uuid;

    // #[tokio::test]
    // async fn test_post() {
    //     let bna_uuid = Uuid::new_v4();
    //     dbg!(&bna_uuid);
    //     let bna_post = BNAPost {
    //         core_services: BNACoreServices {
    //             dentists: Some(0.0),
    //             doctors: Some(0.0),
    //             grocery: Some(1.69),
    //             hospitals: Some(5.18),
    //             pharmacies: Some(0.0),
    //             social_services: Some(0.0),
    //             score: Some(0.0),
    //         },
    //         features: BNAFeatures {
    //             people: Some(19.17),
    //             retail: Some(0.0),
    //             transit: Some(0.0),
    //         },
    //         infrastructure: BNAInfrastructure {
    //             low_stress_miles: Some(9.3),
    //             high_stress_miles: Some(64.5),
    //         },
    //         opportunity: BNAOpportunity {
    //             employment: Some(8.26),
    //             higher_education: Some(0.0),
    //             k12_education: Some(8.31),
    //             technical_vocational_college: Some(0.0),
    //             score: Some(0.0),
    //         },
    //         recreation: BNARecreation {
    //             community_centers: Some(0.0),
    //             parks: Some(7.13),
    //             recreation_trails: Some(0.0),
    //             score: Some(0.0),
    //         },
    //         summary: BNASummary {
    //             bna_uuid,
    //             version: "24.05".to_string(),
    //             city_id: Uuid::parse_str("02fa7cef-bfb9-494e-9ae9-cdbaeb15f11f").unwrap(),
    //             score: 0.0,
    //         },
    //     };
    //     let body = serde_json::to_string(&bna_post).unwrap();
    //     let event = http::Request::builder()
    //         .header(http::header::CONTENT_TYPE, "application/json")
    //         .body(Body::from(body))
    //         .expect("failed to build request")
    //         .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
    //             lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    //         ));
    //     let r = function_handler(event).await.unwrap();
    //     dbg!(r);
    // }
}
