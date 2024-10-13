use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use effortless::api::PaginationParameters;
use entity::wrappers::city::CityPost;
use lambda_http::tracing;
use lambdas::cities::{
    mapper::{
        get_cities_adaptor, get_cities_censuses_adaptor, get_city_adaptor, post_cities_adaptor,
    },
    CitiesPathParameters, ExecutionError,
};
use serde_json::{json, Value};
use std::env::set_var;

#[tokio::main]
async fn main() {
    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/cities", get(get_cities).post(post_cities))
        .route("/cities/:country/:region/:name", get(get_city))
        .route(
            "/cities/:country/:region/:name/census",
            get(get_city_censuses),
        )
        .route(
            "/cities/:country/:region/:name/ratings",
            get(get_city_censuses),
        );

    // run(app).await
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_city(params: Path<CitiesPathParameters>) -> Result<Json<Value>, ExecutionError> {
    let Path(params) = params;
    get_city_adaptor(&params.country, &params.region, &params.name)
        .await
        .map(|v| Json(v))
}

async fn get_cities(
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_adaptor(pagination.page, pagination.page_size())
        .await
        .map(|v| Json(json!(v.payload())))
}

async fn get_city_censuses(
    params: Path<CitiesPathParameters>,
    pagination: Option<Query<PaginationParameters>>,
) -> Result<Json<Value>, ExecutionError> {
    let Path(params) = params;
    let Query(pagination) = pagination.unwrap_or_default();
    get_cities_censuses_adaptor(
        &params.country,
        &params.region,
        &params.name,
        pagination.page,
        pagination.page_size(),
    )
    .await
    .map(|v| Json(json!(v.payload())))
}

async fn post_cities(city: Json<CityPost>) -> Result<Json<Value>, ExecutionError> {
    let Json(city) = city;
    post_cities_adaptor(city).await.map(|v| Json(v))
}
