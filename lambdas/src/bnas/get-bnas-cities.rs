use dotenv::dotenv;
use entity::{bna, city};
use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response,
};
use lambdas::{database_connect, get_apigw_request_id, APIError, APIErrorSource, APIErrors};
use sea_orm::{prelude::Uuid, EntityTrait};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID")).await?;

    // Retrieve a bna result and its related city.
    let apigw_request_id = get_apigw_request_id(&event);
    match event.path_parameters().first("bna_id") {
        Some(bna_id_str) => {
            let bna_id = bna_id_str.parse::<Uuid>();
            match bna_id {
                Ok(bna_id) => {
                    let model = bna::Entity::find_by_id(bna_id)
                        .find_also_related(city::Entity)
                        .one(&db)
                        .await?;
                    let res: Response<Body> = match model {
                        None => {
                            let api_error = APIError::new(
                                apigw_request_id,
                                StatusCode::NOT_FOUND,
                                String::from("Content Not Found"),
                                format!("BNA entry with the id {bna_id} was not found."),
                                APIErrorSource::Pointer(event.uri().path().to_string()),
                            );
                            APIErrors::new(&[api_error]).into()
                        }
                        Some(model) => json!(model).into_response().await,
                    };
                    Ok(res)
                }
                Err(e) => {
                    let api_error = APIError::with_parameter(
                        apigw_request_id,
                        "bna_id",
                        format!("{bna_id_str} is not a valid UUID: {e}").as_str(),
                    );
                    Ok(APIErrors::new(&[api_error]).into())
                }
            }
        }
        None => {
            let api_error =
                APIError::with_parameter(apigw_request_id, "bna_id", "Parameter is missing.");
            Ok(APIErrors::new(&[api_error]).into())
        }
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

    run(service_fn(function_handler)).await
}
