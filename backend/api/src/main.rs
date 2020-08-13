use jwkkeys::JwkConfiguration;
use std::thread;

use ji_cloud_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();
    logger::init_logger();

    let settings = core::settings::init().await?;

    let jwk_verifier = jwkkeys::create_verifier(JwkConfiguration {
        audience: settings.jwk_audience.clone(),
        issuer: settings.jwk_issuer.clone(),
    })
    .await?;

    let _ = jwkkeys::run_task(jwk_verifier.clone());

    let client = s3::S3Client::from_env()?;

    let db_pool = db::get_pool(settings.connect_options.clone()).await?;

    let handle = thread::spawn(|| http::run(db_pool, settings, jwk_verifier, client));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
