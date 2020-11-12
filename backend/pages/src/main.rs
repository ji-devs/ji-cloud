use core::settings::{self, SettingsManager};

mod logger;
mod server;
mod templates;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    logger::init_logger()?;

    let remote_target = settings::read_remote_target()?;

    let (_guard, settings) = {
        let settings = SettingsManager::new(remote_target).await?;

        let guard = core::sentry::init(&settings.sentry_pages_key().await?, remote_target)?;

        (guard, settings.runtime_settings().await?)
    };

    server::run(settings).await
}
