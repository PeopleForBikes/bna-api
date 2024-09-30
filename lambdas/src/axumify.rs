// Fully expirmental rewrite of the API using Axum.
//
// The name comes from chatGPT who said:
//  Axumensia
//    Purpose: This spell enhances memory, allowing the caster to recall even the most obscure details.
//    Incantation: "Axumensia Memor!"

use crate::database_connect;
use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Router};
use dotenv::dotenv;
use effortless::error::APIError;
use entity::city;
use lambda_http::tracing;
use sea_orm::EntityTrait;
use std::env::set_var;

enum AxumensiaError {
    NotFound,
}

#[tokio::main]
async fn main() {
    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // let app = Router::new().route("/cities/:country/:region/:name", get(get_city));

    // run(app).await

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

async fn get_city(
    Path((country, region, name)): Path<(String, String, String)>,
) -> Result<Json<city::Model>, APIError> {
    dotenv().ok();

    // Set the database connection.
    let db = database_connect(Some("DATABASE_URL_SECRET_ID"))
        .await
        .unwrap();

    let select = city::Entity::find_by_id((country, region, name));
    let model = select.one(&db).await.unwrap();
    match model {
        Some(model) => Ok(Json(model)),
        None => Err(APIError::not_found(
            Some("id".to_string()),
            "source",
            "message",
        )),
    }
}
