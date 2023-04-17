use lambda_http::{Request, RequestExt};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{env, num::ParseIntError};

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

/// Set the database connection.
///
/// Look up for the connection string first inside the value of the `DATABASE_URL`
/// environment variable, then in the AWS Secrets Manager with the `secret_id` id
/// if provided.
pub async fn database_connect(secret_id: Option<String>) -> Result<DatabaseConnection, DbErr> {
    let database_url: String = match env::var("DATABASE_URL") {
        Ok(value) => value,
        Err(_) => match secret_id {
            Some(secret_id) =>
                match env::var(&secret_id) {
                  Ok(v) => get_aws_secrets(&v).await.map_err(|e| DbErr::Custom(e.to_string()))?,
                  Err(_) => return Err(DbErr::Custom(format!("Cannot find the connection string in the AWS Secrets Manager. Ensure `{:?}` is correctly set.", secret_id))),
            }
            None => return Err(DbErr::Custom(format!("Cannot find the connection string. Ensure `DATABASE_URL` is correctly set."))),
        },
    };
    Ok(Database::connect(database_url).await?)
}

/// Retrieve a secret from the AWS Secrets Manager.
///
/// Ref: https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieving-secrets_lambda.html
pub async fn get_aws_secrets(secret_id: &str) -> Result<String, reqwest::Error> {
    let aws_session_token = env::var("AWS_SESSION_TOKEN")
        .expect("The caller is supposed to be within the Lambda environment.");
    Ok(reqwest::Client::new()
        .get(format!(
            "http://localhost:2773/secretsmanager/get?secretId={secret_id}"
        ))
        .header("X-Aws-Parameters-Secrets-Token", aws_session_token)
        .send()
        .await?
        .text()
        .await?)
}
