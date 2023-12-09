use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use color_eyre::{eyre::Report, Result};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    list_objects(&s3_client, "brokenspoke-analyzer").await?;

    Ok(())
}

pub async fn list_objects(client: &Client, bucket_name: &str) -> Result<Vec<String>> {
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    println!("Objects in bucket:");
    // for obj in objects.contents() {
    //     if obj.key.clone().unwrap().ends_with("/") {
    //         println!("{:?}", obj.key().unwrap());
    //     }
    // }
    let mut objs = objects
        .contents()
        .iter()
        .filter(|o| o.key.clone().unwrap().ends_with("/"))
        .map(|o| o.key.clone().unwrap())
        .collect::<Vec<String>>();
    objs.sort();

    Ok(objs)
}
