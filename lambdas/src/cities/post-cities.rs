use dotenv::dotenv;
use effortless::api::{invalid_body, parse_request_body};
use entity::{
    country,
    prelude::*,
    wrappers::{self, city::CityPost},
};
use lambda_http::{
    http::{header, StatusCode},
    run, service_fn, Body, Error, Request, Response,
};
use lambdas::database_connect;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
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

    // Extract some country information.
    let country = wrapper.country.clone();
    let state_full = wrapper.state.clone();
    let region = wrapper.region.clone();

    // Turn the model wrapper into an active model.
    let mut active_city = wrapper.into_active_model();

    // Assign a city_id.
    active_city.id = ActiveValue::Set(Uuid::new_v4());

    // Get the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Ensure the country is a valid one.
    if Country::find()
        .filter(country::Column::Name.eq(&country))
        .one(&db)
        .await?
        .is_none()
    {
        return Ok(invalid_body(
            &event,
            "the country `{country}` is not in the list of countries supported by the BNA",
        )
        .into());
    }

    // If the country is the United States, set the region to the standardized state abbreviation.
    if country.to_lowercase().eq("united states") {
        match StateRegionCrosswalk::find_by_id(state_full)
            .one(&db)
            .await?
        {
            Some(model) => {
                let region: wrappers::BnaRegion = model.region.into();
                active_city.region = ActiveValue::Set(Some(region.to_string()));
            }
            None => return Ok(invalid_body(&event, "invalid state: {state_full}").into()),
        }
    }

    // If the region is not set, ensure it is equal to the country.
    if region.is_none() {
        active_city.region = ActiveValue::Set(Some(country));
    }

    // And insert a new entry.
    info!("inserting City into database: {:?}", active_city);
    let city = active_city.insert(&db).await?;
    let response = Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::Text(json!(city).to_string()))
        .expect("unable to build http::Response");
    Ok(response)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use lambda_http::{http, RequestExt};

    // #[tokio::test]
    // async fn test_handler() {
    //     let event = http::Request::builder()
    //   .header(http::header::CONTENT_TYPE, "application/json")
    //   .body(Body::from(r#"{"name": "santa rosa 000","country": "united states","fips_code": "3570670","state": "new mexico", "state_abbrev": "NM"}"#))
    //   .expect("failed to build request")
    //   .with_request_context(lambda_http::request::RequestContext::ApiGatewayV2(lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext::default()));

    //     let r = function_handler(event).await.unwrap();
    //     dbg!(r);
    // }
}
