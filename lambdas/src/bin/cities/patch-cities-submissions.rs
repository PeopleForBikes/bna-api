use dotenv::dotenv;
use effortless::{
    api::{missing_parameter, parse_path_parameter, parse_request_body},
    error::APIErrors,
};
use entity::wrappers::submission::SubmissionPatch;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::core::resource::cities::adaptor::patch_cities_submission_adaptor;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve the ID of the Submission to update.
    let parameter = "submission_id";
    let submission_id = match parse_path_parameter::<i32>(&event, parameter) {
        Ok(id) => match id {
            Some(id) => id,
            None => return Ok(missing_parameter(&event, parameter).into()),
        },
        Err(e) => return Ok(e.into()),
    };

    // Prepare the model to update.
    let submission = match parse_request_body::<SubmissionPatch>(&event) {
        Ok(v) => v,
        Err(e) => return Ok(e.into()),
    };

    match patch_cities_submission_adaptor(submission_id, submission).await {
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

//     #[test]
//     fn test_prepare_active_model() {
//         let event = http::Request::builder()
//             .header(http::header::CONTENT_TYPE, "application/json")
//             .body(Body::from(r#"{"city": "santa rosa","country": "usa","email": "jane.dpe@orgllc.com","fips_code": "3570670","first_name": "Jane","last_name": "Doe","organization": "Organization LLC","region": "new mexico","title": "CTO","consent": true}"#))
//             .expect("failed to build request")
//             .with_path_parameters(HashMap::from([("submission_id".to_string(), "1".to_string())]))
//             .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default()));
//         let active_submission = prepare_active_model(&event).unwrap();
//         assert_eq!(
//             active_submission.country,
//             sea_orm::ActiveValue::Set("usa".to_string())
//         )
//     }
// }
