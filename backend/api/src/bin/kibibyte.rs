use core::settings::req_env;
use ji_cloud_api::*;
use rusoto_core::{credential::EnvironmentProvider, HttpClient, Region};
use rusoto_s3::{CreateBucketRequest, S3};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();
    logger::init_logger();

    log::info!("initializing s3 client");
    let endpoint = req_env("S3_ENDPOINT")?;
    let bucket = req_env("S3_BUCKET")?;

    let region = Region::Custom {
        name: "auto".to_owned(),
        endpoint,
    };

    let credentials = EnvironmentProvider::with_prefix("S3");
    let dispatcher = HttpClient::new()?;
    let client = rusoto_s3::S3Client::new_with(dispatcher, credentials, region);

    log::info!("Creating bucket: {}", bucket);

    client
        .create_bucket(CreateBucketRequest {
            bucket,
            ..CreateBucketRequest::default()
        })
        .await?;

    log::info!("have a nice day!");

    Ok(())
}
