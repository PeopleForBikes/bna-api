use dotenv::dotenv;
use effortless::{
    api::{entry_not_found, extract_pagination_parameters},
    error::APIErrors,
    fragment::BnaRequestExt,
};
use entity::{
    core_services, infrastructure, opportunity, people, prelude::*, recreation, retail, transit,
};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    core::resource::ratings::{
        adaptor::get_ratings_summaries_adaptor, extract_path_parameters, extract_query_parameters,
        BNAComponent, BNAPathParameters, BNAQueryParameters,
    },
    database_connect,
};
use sea_orm::{prelude::Uuid, EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};

#[derive(FromQueryResult, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BNA {
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

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Get the database connection.
    // let db = match api_database_connect(&event).await {
    //     Ok(db) => db,
    //     Err(e) => return Ok(e),
    // };
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve all bnas or a specific one.
    debug!("Processing the requests...");

    // With params.
    if event.has_path_parameters() {
        let params: BNAPathParameters = match extract_path_parameters(&event) {
            Ok(p) => p,
            Err(e) => return Ok(e.into()),
        };

        let queries: BNAQueryParameters = match extract_query_parameters(&event) {
            Ok(a) => a,
            Err(e) => return Ok(e.into()),
        };

        let select = Summary::find_by_id(params.bna_id);
        let res = match queries.component {
            BNAComponent::All => {
                let model = select
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
                    .into_model::<BNA>()
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::Summary => {
                let model = select.clone().one(&db).await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::Infratructure => {
                let model = select
                    .clone()
                    .find_also_related(infrastructure::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::Recreation => {
                let model = select
                    .clone()
                    .find_also_related(recreation::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::Opportunity => {
                let model = select
                    .clone()
                    .find_also_related(opportunity::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::CoreServices => {
                let model = select
                    .clone()
                    .find_also_related(core_services::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::People => {
                let model = select
                    .clone()
                    .find_also_related(people::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::Retail => {
                let model = select
                    .clone()
                    .find_also_related(retail::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
            BNAComponent::Transit => {
                let model = select
                    .clone()
                    .find_also_related(transit::Entity)
                    .one(&db)
                    .await?;
                match model {
                    Some(model) => json!(model).into_response().await,
                    None => entry_not_found(&event).into(),
                }
            }
        };
        return Ok(res);
    }

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Retrieve entries.
    match get_ratings_summaries_adaptor(pagination.page, pagination.page_size()).await {
        Ok(v) => Ok(v.payload().into_response().await),
        Err(e) => Ok(APIErrors::from(e).into()),
    }
}

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

#[cfg(test)]
mod tests {

    // use super::*;
    // use aws_lambda_events::http;
    // use lambda_http::RequestExt;
    // use std::collections::HashMap;

    // #[tokio::test]
    // async fn test_handler_all() {
    //     let event = http::Request::builder()
    //         .header(http::header::CONTENT_TYPE, "application/json")
    //         .body(Body::Empty)
    //         .expect("failed to build request")
    //         .with_path_parameters(HashMap::from([(
    //             "bna_id".to_string(),
    //             "837082b8-c8a0-469e-b310-c868d7f140a2".to_string(), // Santa Monica, CA
    //         )]))
    //         .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
    //             lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    //         ));
    //     let r = function_handler(event).await.unwrap();
    //     dbg!(r);
    // }

    //     #[tokio::test]
    //     async fn test_handler_opportunity() {
    //         let event = http::Request::builder()
    //             .header(http::header::CONTENT_TYPE, "application/json")
    //             .body(Body::Empty)
    //             .expect("failed to build request")
    //             .with_path_parameters(HashMap::from([(
    //                 "id".to_string(),
    //                 "837082b8-c8a0-469e-b310-c868d7f140a2".to_string(), // Santa Monica, CA
    //             )]))
    //             .with_query_string_parameters(HashMap::from([(
    //                 "component".to_string(),
    //                 "Opportunity".to_string(),
    //             )]))
    //             .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
    //                 lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    //             ));
    //         let r = function_handler(event).await.unwrap();
    //         dbg!(r);
    //     }
}
