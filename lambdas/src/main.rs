use ::tracing::{debug, info};
use lambda_http::{run, tracing, Error};
use lambdas::{
    core::resource::{
        cities, pipelines, price, ratings, reports,
        schema::{APIError, APIErrorSource, APIErrors, OrderDirection},
        usstates,
    },
    database_connect, DB_CONN,
};
use std::{
    env::{self, set_var},
    fs,
};
use tower_http::trace::TraceLayer;
use utoipa::{
    openapi::{Components, ContactBuilder, Info, OpenApiBuilder, Server, Tag},
    schema,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    // Check for user-defined log level.
    let log_level = match env::var("BNA_API_LOG_LEVEL") {
        Ok(v) => match v.to_ascii_lowercase().as_str() {
            "debug" => tracing::Level::DEBUG,
            _ => tracing::Level::INFO,
        },
        Err(_) => tracing::Level::INFO,
    };

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    debug!(loglevel= ?log_level);

    // Define the OpenApi Specification.
    let api = OpenApiBuilder::new()
        .info(
            Info::builder()
                .title("BNA REST API")
                .version("1.0.0")
                .description(Some(
                    "Provides a way to retrieve the BNA results programmatically.",
                ))
                .contact(Some(
                    ContactBuilder::new()
                        .name(Some("The BNA Mechanics team"))
                        .url(Some("https://peopleforbikes.github.io/"))
                        .build(),
                ))
                .build(),
        )
        .servers(Some(vec![
            Server::builder()
                .description(Some("Local development API"))
                .url("http://localhost:3000")
                .build(),
            Server::builder()
                .description(Some("Staging API"))
                .url("https://api.peopleforbikes.xyz")
                .build(),
        ]))
        .tags(Some(vec![
            Tag::builder()
                .name("city")
                .description(Some("City API endpoints"))
                .build(),
            Tag::builder()
                .name("pipeline")
                .description(Some("Pipeline API endpoints"))
                .build(),
            Tag::builder()
                .name("price")
                .description(Some("Price API endpoints"))
                .build(),
            Tag::builder()
                .name("rating")
                .description(Some("Rating API endpoints"))
                .build(),
        ]))
        .components(Some(
            Components::builder()
                .schema(
                    "APIError",
                    schema!(
                        #[inline]
                        APIError
                    ),
                )
                .schema(
                    "APIErrorSource",
                    schema!(
                        #[inline]
                        APIErrorSource
                    ),
                )
                .schema(
                    "APIErrors",
                    schema!(
                        #[inline]
                        APIErrors
                    ),
                )
                .schema(
                    "OrderDirection",
                    schema!(
                        #[inline]
                        OrderDirection
                    ),
                )
                .build(),
        ))
        .build();

    // Create the app router.
    let (app, api) = OpenApiRouter::with_openapi(api)
        .merge(cities::endpoint::routes())
        .merge(pipelines::endpoint::routes())
        .merge(price::endpoint::routes())
        .merge(ratings::endpoint::routes())
        .merge(reports::endpoint::routes())
        .merge(usstates::endpoint::routes())
        .layer(TraceLayer::new_for_http())
        .split_for_parts();

    // Write the specification file to disk.
    if env::var("BNA_API_GENERATE_ONLY")
        .ok()
        .is_some_and(|v| v == *"1")
    {
        info!("Regenerating the OpenAPI specification file.");
        let _ = fs::write("./openapi-3.1.yaml", api.to_yaml().unwrap());
        return Ok(());
    }

    // Add the Swagger UI.
    let app = app.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    // Set the database connection once at startup.
    let db = database_connect().await?;
    DB_CONN.set(db.clone()).ok();

    // Lookup for the  standalone flag.
    let standalone = env::var("BNA_API_STANDALONE")
        .ok()
        .is_some_and(|v| v == *"1");

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
