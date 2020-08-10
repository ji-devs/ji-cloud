use jwkkeys::JwkConfiguration;
use std::thread;

mod db;
mod extractor;
mod http;
mod jwkkeys;
mod jwt;
mod logger;

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

    let db_pool = db::get_pool(settings.connect_options.clone()).await?;

    let handle = thread::spawn(|| http::run(db_pool, settings, jwk_verifier));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
