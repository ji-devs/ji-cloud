use core::settings::SettingsManager;

mod logger;
mod server;
mod templates;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    logger::init_logger()?;

    let settings: SettingsManager = core::settings::SettingsManager::new().await?;

    server::run(settings.runtime_settings().await?).await
}
