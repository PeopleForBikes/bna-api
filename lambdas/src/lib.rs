use lambda_http::{Request, RequestExt};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, num::ParseIntError};

/// Represent the contents of the encrypted fields SecretString or SecretBinary
/// from the specified version of a secret, whichever contains content.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretValue {
    /// Amazon Resource Name of the secret.
    #[serde(alias = "ARN")]
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
    /// Metadata.
    pub result_metadata: HashMap<String, String>,
}

/// Retrieve the pagination parameters.
///
/// If nothing is provided, the first page is returned and will contain up to 50 items.
pub fn pagination_parameters(event: &Request) -> Result<(u64, u64), ParseIntError> {
    let page_size = event
        .query_string_parameters()
        .first("page_size")
        .unwrap_or("50")
        .parse::<u64>()?;
    let page = event
        .query_string_parameters()
        .first("page")
        .unwrap_or("0")
        .parse::<u64>()?;
    Ok((page_size, page))
}

/// Return the database connection.
///
/// Look up for the connection string:
///   - first inside the value of the `DATABASE_URL`environment variable,
///   - then in the AWS Secrets Manager with the `secret_id` id if provided.
pub async fn database_connect(secret_id: Option<&str>) -> Result<DatabaseConnection, DbErr> {
    const DATABASE_URL_KEY: &str = "DATABASE_URL";
    let database_url: String = match env::var(DATABASE_URL_KEY) {
        Ok(value) => value,
        Err(_) => match secret_id {
            Some(secret_id) =>
                match env::var(secret_id) {
                  Ok(v) => {
                    let secret_value = get_aws_secrets(&v)
                        .await
                        .map_err( DbErr::Custom)?;
                    let secrets: HashMap<String, String> = serde_json::from_str(&secret_value.secret_string)
                        .map_err(|e| DbErr::Custom(format!("Cannot deserialize the cached secret: {e}")))?;
                    match secrets.get(DATABASE_URL_KEY) {
                      Some(v) => v.to_owned(),
                      None => return Err(DbErr::Custom(format!("Cannot find the connection string within the secret {:?}. Ensure `{DATABASE_URL_KEY}` is correctly set.", secret_id))),
                    }
                  },
                  Err(e) => return Err(DbErr::Custom(format!("Cannot find the connection string in the AWS Secrets Manager. Ensure `{:?}` is correctly set. Reason: {e}", secret_id))),
            }
            None => return Err(DbErr::Custom(format!("Cannot find the connection string. Ensure `{DATABASE_URL_KEY}` is correctly set."))),
        },
    };
    Database::connect(database_url).await
}

/// Retrieve a secret from the AWS Secrets Manager using the Lambda caching layer.
///
/// Ref: https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieving-secrets_lambda.html
pub async fn get_aws_secrets(secret_id: &str) -> Result<SecretValue, String> {
    let aws_session_token =
        env::var("AWS_SESSION_TOKEN").map_err(|e| format!("Cannot find AWS session token: {e}"))?;
    reqwest::Client::new()
        .get(format!(
            "http://localhost:2773/secretsmanager/get?secretId={secret_id}"
        ))
        .header("X-Aws-Parameters-Secrets-Token", aws_session_token)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<SecretValue>()
        .await
        .map_err(|e| e.to_string())
}
