use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::Value;
use std::env;
use tracing::info;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SecretValue {
    /// Amazon Resource Name of the secret.
    #[serde(rename = "ARN")]
    pub arn: String,
    /// Creation date.
    pub created_date: String,
    /// The friendly name of the secret.
    pub name: String,
    /// The decrypted secret value, if the secret value was originally provided
    /// as binary data in the form of a byte array. The response parameter
    /// represents the binary data as a base64-encoded string.
    ///
    /// If the secret was created by using the Secrets Manager console, or if
    /// the secret value was originally provided as a string, then this field
    /// is omitted. The secret value appears in SecretString instead.
    pub secret_binary: Option<String>,
    /// The decrypted secret value, if the secret value was originally provided
    /// as a string or through the Secrets Manager console.
    /// If this secret was created by using the console, then Secrets Manager
    /// stores the information as a JSON structure of key/value pairs.
    pub secret_string: String,
    /// Unique identifier of the version of the secret.
    pub version_id: String,
    /// A list of all of the staging labels currently attached to this version
    /// of the secret.
    pub version_stages: Vec<String>,
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

    info!("Fetching secrets from main");
    get_aws_secrets("DATABASE_URL").await;
    run(service_fn(function_handler)).await.map_err(|e| {
        info!("{e}");
        e
    })
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
pub(crate) async fn function_handler(_event: LambdaEvent<Value>) -> Result<(), Error> {
    // Extract some useful information from the request
    // let payload = event.payload;
    // tracing::info!("Payload: {:?}", payload);
    info!("Fetching secrets from function_handler");
    get_aws_secrets("DATABASE_URL").await;

    Ok(())
}

/// Retrieve a secret from the AWS Secrets Manager using the Lambda caching layer.
///
/// Ref: <https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieving-secrets_lambda.html>
pub async fn get_aws_secrets(secret_id: &str) -> () {
    let aws_session_token = env::var("AWS_SESSION_TOKEN").unwrap();
    let res = reqwest::Client::new()
        .get(format!(
            "http://localhost:2773/secretsmanager/get?secretId={secret_id}"
        ))
        .header("X-Aws-Parameters-Secrets-Token", aws_session_token)
        .send()
        .await;
    dbg!(&res);
    let res = res.unwrap().json::<SecretValue>().await;
    dbg!(&res);

    // match res {
    //     Ok(res) => Ok(res.json::<SecretValue>().await?),

    //     Err(err) => match err.status() {
    //         Some(StatusCode::NOT_FOUND) => Err(format!("Secret not found: {}", secret_id)),
    //         _ => Err(err.to_string()),
    //     },
    // }
}
