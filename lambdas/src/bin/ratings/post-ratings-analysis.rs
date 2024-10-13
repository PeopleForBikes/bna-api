use dotenv::dotenv;
use effortless::api::parse_request_body;
use entity::{prelude::*, wrappers::bna_pipeline::BNAPipelinePost};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{EntityTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract and serialize the data.
    let wrapper = match parse_request_body::<BNAPipelinePost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Turn the model wrapper into an active model.
    let active_model = wrapper.into_active_model();

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // And insert a new entry.
    info!(
        "inserting Brokenspoke pipeline into database: {:?}",
        active_model
    );
    let res = BnaPipeline::insert(active_model).exec(&db).await?;
    Ok(json!(res.last_insert_id).into_response().await)
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

    #[tokio::test]
    async fn test_handler_all() {
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
            .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
                lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
            ));
        let r = function_handler(event).await;
        let _var_name = dbg!(r);
    }
}
