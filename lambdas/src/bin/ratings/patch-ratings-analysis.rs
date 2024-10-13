use dotenv::dotenv;
use effortless::{
    api::{invalid_path_parameter, missing_parameter, parse_request_body},
    error::APIErrors,
    fragment::BnaRequestExt,
};
use entity::{bna_pipeline::ActiveModel, prelude::*, wrappers::bna_pipeline::BNAPipelinePatch};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{prelude::Uuid, ActiveValue, EntityTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Prepare the model to update.
    let active_model = match prepare_active_model(&event) {
        Ok(model) => model,
        Err(e) => return Ok(e.into()),
    };

    info!(
        "updating Submission {:?} into database: {:?}",
        active_model.state_machine_id, active_model
    );

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Update the entry.
    let res = BnaPipeline::update(active_model).exec(&db).await?;
    Ok(json!(res.state_machine_id).into_response().await)
}

pub fn prepare_active_model(event: &Request) -> Result<ActiveModel, APIErrors> {
    // Retrieve the ID of the entry to update.
    let parameter = "analysis_id";
    let id = event
        .path_parameter::<Uuid>(parameter)
        .ok_or(missing_parameter(event, parameter))?
        .map_err(|e| invalid_path_parameter(event, parameter, e.to_string().as_str()))?;

    // Extract and deserialize the data.
    let wrapper = parse_request_body::<BNAPipelinePatch>(event)?;

    // Turn the wrapper into an active model.
    let mut active_model = wrapper.into_active_model();
    active_model.state_machine_id = ActiveValue::Unchanged(id);
    Ok(active_model)
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

    use super::*;
    use aws_lambda_events::http;
    use lambda_http::RequestExt;
    use std::collections::HashMap;

    #[test]
    fn test_prepare_active_model() {
        let event = http::Request::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::Text(
                r#"{
                  "cost": null,
                  "end_time": null,
                  "fargate_price_id": null,
                  "fargate_task_arn": null,
                  "result_posted": null,
                  "s3_bucket": null,
                  "sqs_message": null,
                  "start_time": [
                    2024,
                    247,
                    13,
                    7,
                    29,
                    922293000,
                    0,
                    0,
                    0
                  ],
                  "state_machine_id": "fc009967-c4d0-416b-baee-93708ac80cbc",
                  "status": null,
                  "step": "Analysis",
                  "torn_down": null
                }"#
                .to_string(),
            ))
            .expect("failed to build request")
            .with_path_parameters(HashMap::from([(
                "analysis_id".to_string(),
                "fc009967-c4d0-416b-baee-93708ac80cbc".to_string(), // Santa Monica, CA
            )]))
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let r = prepare_active_model(&event);
        assert!(r.is_ok());
    }
}
