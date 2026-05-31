use ji_core::settings::{self, SettingsManager};

mod db;
mod logger;
mod server;
mod templates;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    logger::init()?;

    let remote_target = settings::read_remote_target()?;

    let (settings, db_pool) = {
        let settings = SettingsManager::new(remote_target).await?;

        let runtime_settings = settings.runtime_settings().await?;
        let db_pool = db::get_pool(
            settings
                .db_connect_options(settings::read_sql_proxy())
                .await?,
        )
        .await?;

        (runtime_settings, db_pool)
    };

    server::run(settings, db_pool).await
}
