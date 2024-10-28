use axum::Router;
use lambda_http::{run, tracing, Error};
use lambdas::core::resource::{cities, price, ratings};
use std::env::{self, set_var};

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    // Create the app router.
    let app = Router::new()
        .merge(cities::endpoint::routes())
        .merge(price::endpoint::routes())
        .merge(ratings::endpoint::routes());

    // Lookup for the  standalone flag.
    let standalone = env::var("BNA_API_STANDALONE")
        .ok()
        .map_or(false, |v| v == *"1");

    // Start the server in standalone mode or in lambda_http mode.
    if standalone {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
        Ok(())
    } else {
        run(app).await
    }
}
