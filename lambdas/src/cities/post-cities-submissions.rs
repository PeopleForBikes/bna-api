use dotenv::dotenv;
use effortless::api::{invalid_body, parse_request_body};
use entity::wrappers::submission::SubmissionPost;
use lambda_http::{
    http::{header, StatusCode},
    run, service_fn, Body, Error, Request, Response,
};
use lambdas::{database_connect, db::find_country};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serde_json::json;
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Extract and serialize the data.
    let wrapper = match parse_request_body::<SubmissionPost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };
    let country = wrapper.country.clone();

    // Turn the model wrapper into an active model.
    let mut active_submission = wrapper.into_active_model();

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Add the status.
    active_submission.status = ActiveValue::Set("Pending".to_string());

    // Ensure the country is a valid one.
    let country_found = find_country(&db, &country).await?;
    dbg!(country_found);
    if !find_country(&db, &country).await? {
        return Ok(invalid_body(
            &event,
            format!("the country `{country}` is not in the list of countries supported by the BNA")
                .as_str(),
        )
        .into());
    }

    // And insert a new entry.
    info!(
        "inserting Submission into database: {:?}",
        active_submission
    );
    let submission = match active_submission.insert(&db).await {
        Ok(m) => m,
        Err(e) => return Ok(invalid_body(&event, e.to_string().as_str()).into()),
    };
    let response = Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::Text(json!(submission).to_string()))
        .expect("unable to build http::Response");
    Ok(response)
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

    //     use super::*;
    //     use aws_lambda_events::http;
    //     use lambda_http::RequestExt;

    //     #[tokio::test]
    //     async fn test_handler() {
    //         let event = http::Request::builder()
    //             .header(http::header::CONTENT_TYPE, "application/json")
    //             .body(Body::Text(
    //                 r#"{
    //   "city": "santa rosa",
    //   "country": "United States",
    //   "email": "jane.dpe@orgllc.com",
    //   "fips_code": "3570670",
    //   "first_name": "Jane",
    //   "last_name": "Doe",
    //   "organization": "Organization LLC",
    //   "region": "new mexico",
    //   "occupation": "CTO",
    //   "consent": true
    // }"#
    //                 .to_string(),
    //             ))
    //             .expect("failed to build request")
    //             .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(
    //                 lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default(),
    //             ));
    //         let r = function_handler(event).await.unwrap();
    //         dbg!(r);
    //     }
}
