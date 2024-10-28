use dotenv::dotenv;
use effortless::{
    api::{missing_parameter, parse_path_parameter, parse_request_body},
    error::APIErrors,
};
use entity::wrappers::bna_pipeline::BNAPipelinePatch;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::core::resource::ratings::adaptor::patch_ratings_analysis_adaptor;
use sea_orm::prelude::Uuid;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract and deserialize the data.
    let wrapper = match parse_request_body::<BNAPipelinePatch>(&event) {
        Ok(v) => v,
        Err(e) => return Ok(e.into()),
    };

    // Retrieve the ID of the entry to update.
    let parameter = "analysis_id";
    let analysis_id = match parse_path_parameter::<Uuid>(&event, parameter) {
        Ok(id) => match id {
            Some(id) => id,
            None => return Ok(missing_parameter(&event, parameter).into()),
        },
        Err(e) => return Ok(e.into()),
    };

    match patch_ratings_analysis_adaptor(wrapper, analysis_id).await {
        Ok(v) => Ok(v.into_response().await),
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

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use aws_lambda_events::http;
//     use lambda_http::RequestExt;
//     use std::collections::HashMap;

// #[test]
// fn test_prepare_active_model() {
//     let event = http::Request::builder()
//         .header(http::header::CONTENT_TYPE, "application/json")
//         .body(Body::Text(
//             r#"{
//               "cost": null,
//               "end_time": null,
//               "fargate_price_id": null,
//               "fargate_task_arn": null,
//               "result_posted": null,
//               "s3_bucket": null,
//               "sqs_message": null,
//               "start_time": [
//                 2024,
//                 247,
//                 13,
//                 7,
//                 29,
//                 922293000,
//                 0,
//                 0,
//                 0
//               ],
//               "state_machine_id": "fc009967-c4d0-416b-baee-93708ac80cbc",
//               "status": null,
//               "step": "Analysis",
//               "torn_down": null
//             }"#
//             .to_string(),
//         ))
//         .expect("failed to build request")
//         .with_path_parameters(HashMap::from([(
//             "analysis_id".to_string(),
//             "fc009967-c4d0-416b-baee-93708ac80cbc".to_string(), // Santa Monica, CA
//         )]))
//         .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
//             lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
//         ));
//     let r = prepare_active_model(&event);
//     assert!(r.is_ok());
// }
// }
