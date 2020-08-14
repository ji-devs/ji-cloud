use core::settings::Settings;
use ji_cloud_api::*;
use jwkkeys::JwkConfiguration;
use std::thread;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();
    logger::init_logger();

    let Settings { init, runtime } = core::settings::init().await?;

    let jwk_verifier = jwkkeys::create_verifier(JwkConfiguration {
        audience: init.jwk_audience,
        issuer: init.jwk_issuer,
    })
    .await?;

    let _ = jwkkeys::run_task(jwk_verifier.clone());

    let s3_use_client = !(runtime.is_local() && init.s3_disable_local);

    let client = s3::S3Client::new(init.s3_endpoint, init.s3_bucket, s3_use_client)?;

    let db_pool = db::get_pool(init.connect_options).await?;

    let handle = thread::spawn(|| http::run(db_pool, runtime, jwk_verifier, client));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
