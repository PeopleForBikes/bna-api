use dotenv::dotenv;
use effortless::{
    api::{missing_parameter, parse_path_parameter, parse_request_body},
    error::APIErrors,
};
use entity::{prelude::*, submission::ActiveModel, wrappers};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{ActiveValue, EntityTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Prepare the model to update.
    let active_submission = match prepare_active_model(&event) {
        Ok(submission) => submission,
        Err(e) => return Ok(e.into()),
    };

    info!(
        "updating Submission {:?} into database: {:?}",
        active_submission.id, active_submission
    );

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Update the Submission entry.
    let res = Submission::update(active_submission).exec(&db).await?;
    Ok(json!(res.id).into_response().await)
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

    run(service_fn(function_handler)).await
}

pub fn prepare_active_model(event: &Request) -> Result<ActiveModel, APIErrors> {
    // Retrieve the ID of the Submission to update.
    let parameter = "id";
    let submission_id = parse_path_parameter::<i32>(event, parameter)?
        .ok_or(missing_parameter(event, parameter))?;

    // Extract and deserialize the data.
    let wrapper = parse_request_body::<wrappers::SubmissionPatch>(event)?;

    // Turn the wrapper into an active model.
    let mut active_submission = wrapper.into_active_model();
    active_submission.id = ActiveValue::Unchanged(submission_id);
    Ok(active_submission)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::RequestExt;
    use std::collections::HashMap;

    #[test]
    fn test_prepare_active_model() {
        let event = Request::new("{\"city\":\"santa rosa\",\"consent\":true,\"country\":\"usa\",\"email\":\"jane.dpe@orgllc.com\",\"fips_code\":\"3570670\",\"first_name\":\"Jane\",\"id\":1,\"last_name\":\"Doe\",\"organization\":\"Organization LLC\",\"region\":\"new mexico\",\"status\":\"Approved\",\"title\":\"CTO\"}"
        .into()).with_path_parameters(HashMap::from([("id".to_string(), "1".to_string())])).with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
        lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    ));
        let active_submission = prepare_active_model(&event).unwrap();
        assert_eq!(
            active_submission.country,
            sea_orm::ActiveValue::Set("usa".to_string())
        )
    }
}
