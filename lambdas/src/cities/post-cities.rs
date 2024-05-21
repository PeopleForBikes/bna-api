use dotenv::dotenv;
use effortless::api::{invalid_body, parse_request_body};
use entity::{
    prelude::*,
    wrappers::{self, city::CityPost},
};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::database_connect;
use sea_orm::{ActiveValue, EntityTrait, IntoActiveModel};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

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
    let wrapper = match parse_request_body::<CityPost>(&event) {
        Ok(value) => value,
        Err(e) => return Ok(e.into()),
    };

    // Check if the country is US and set the region accordingly.
    let country = wrapper.country.clone();
    let state_full = wrapper.state.clone();
    let region: Option<String> = match country.to_lowercase().eq("United States") {
        true => None,
        false => Some(country),
    };

    // Turn the model wrapper into an active model.
    let mut active_city = wrapper.into_active_model();

    // Assign a city_id.
    active_city.city_id = ActiveValue::Set(Uuid::new_v4());

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Set the region if needed.
    if region.is_none() {
        let state_region_model = StateRegionCrosswalk::find_by_id(state_full)
            .one(&db)
            .await?;
        match state_region_model {
            Some(model) => {
                let region: wrappers::BnaRegion = model.region.into();
                active_city.region = ActiveValue::Set(Some(region.to_string()));
            }
            None => return Ok(invalid_body(&event, "invalid state: {state_full}").into()),
        }
    }

    // And insert a new entry.
    info!("inserting City into database: {:?}", active_city);
    let res = City::insert(active_city).exec(&db).await?;
    Ok(json!(res.last_insert_id).into_response().await)
}

#[cfg(test)]
mod tests {
    use lambda_http::{http, RequestExt};

    use super::*;

    // #[tokio::test]
    // async fn test_handler() {
    //     let event = http::Request::builder()
    //   .header(http::header::CONTENT_TYPE, "application/json")
    //   .body(Body::from(r#"{"name": "santa rosa","country": "united states","fips_code": "3570670","state": "new mexico", "state_abbrev": "NM"}"#))
    //   .expect("failed to build request")
    //   .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default()));

    //     let r = function_handler(event).await.unwrap();
    //     dbg!(r);
    // }
}
