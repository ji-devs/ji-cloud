use core::settings::SettingsManager;
use ji_cloud_api::*;
use std::thread;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();
    logger::init_logger();

    let settings: SettingsManager = SettingsManager::new().await?;

    let runtime = settings.runtime_settings().await?;

    let jwk_verifier = jwkkeys::create_verifier(settings.jwk_settings().await?);

    let _ = jwkkeys::run_task(jwk_verifier.clone());

    let s3 = settings.s3_settings().await?;

    let s3 = s3::S3Client::new(s3.endpoint, s3.bucket, s3.use_client).await?;

    let algolia = algolia::AlgoliaClient::new(settings.algolia_settings().await?)?;

    let db_pool = db::get_pool(settings.db_connect_options().await?).await?;

    let handle = thread::spawn(|| http::run(db_pool, runtime, jwk_verifier, s3, algolia));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
