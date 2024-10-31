use dotenv::dotenv;
use effortless::{
    api::{extract_pagination_parameters, invalid_path_parameter, invalid_query_parameter},
    error::APIErrors,
    fragment::BnaRequestExt,
};
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, Response};
use lambdas::{
    core::resource::price::{
        adaptor::{get_price_fargate_adaptor, get_prices_fargate_adaptor},
        PriceParameters,
    },
    Context, Sort,
};
use tracing::info;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Retrieve pagination parameters if any.
    let pagination = match extract_pagination_parameters(&event) {
        Ok(p) => p,
        Err(e) => return Ok(e),
    };

    // Retrieve price parameter.
    let mut price_parameters = PriceParameters::default();
    let parameter = "sort";
    let sort = event.query_string_parameter::<Sort>(parameter);
    if let Some(sort) = sort {
        match sort {
            Ok(sort) => {
                price_parameters.set_sort(Some(sort));
            }
            Err(e) => {
                let api_error = invalid_query_parameter(
                    &event,
                    parameter,
                    format!("failed to process the `{parameter}` parameter: {e}").as_str(),
                );
                return Ok(APIErrors::new(&[api_error]).into());
            }
        }
    }

    let parameter = "latest";
    let latest = event.query_string_parameter::<bool>(parameter);
    if let Some(latest) = latest {
        match latest {
            Ok(latest) => {
                price_parameters.set_latest(latest);
            }
            Err(e) => {
                let api_error = invalid_query_parameter(
                    &event,
                    parameter,
                    format!("failed to process the `{parameter}` parameter: {e}").as_str(),
                );
                return Ok(APIErrors::new(&[api_error]).into());
            }
        }
    }

    // Retrieve the ID of the entry to retrieve.
    let parameter = "price_id";
    let id = event.path_parameter::<i32>(parameter);

    // Retrieve a specific entry if an id was specified.
    if let Some(id) = id {
        let id = match id {
            Ok(id) => id,
            Err(e) => {
                let api_error = invalid_path_parameter(
                    &event,
                    parameter,
                    format!("failed to process the `{parameter}` parameter: {e}").as_str(),
                );
                return Ok(APIErrors::new(&[api_error]).into());
            }
        };
        let ctx = Context::new(
            event.apigw_request_id(),
            event
                .uri()
                .path_and_query()
                .expect("to have a path and optional query parameters")
                .to_string(),
        );
        match get_price_fargate_adaptor(id, ctx).await {
            Ok(v) => return Ok(v.into_response().await),
            Err(e) => return Ok(APIErrors::from(e).into()),
        };
    }
    // Retrieve all entries.
    match get_prices_fargate_adaptor(price_parameters, pagination.page, pagination.page_size())
        .await
    {
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
