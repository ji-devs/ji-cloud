#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]
#![warn(clippy::pedantic)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::future_not_send)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::string_lit_as_bytes)]
#![warn(clippy::use_self)]
#![warn(clippy::useless_let_if_seq)]

use anyhow::Context;
use core::settings::{self, SettingsManager};
use ji_cloud_api::{algolia, db, http, logger, s3};
use std::thread;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();

    logger::init()?;

    let (runtime_settings, s3, algolia_client, algolia_manager, db_pool, _guard) = {
        log::trace!("initializing settings and processes");
        let remote_target = settings::read_remote_target()?;

        let settings: SettingsManager = SettingsManager::new(remote_target).await?;

        let runtime_settings = settings.runtime_settings().await?;

        let s3 = s3::Client::new(settings.s3_settings().await?)?;

        let algolia_settings = settings.algolia_settings().await?;

        let algolia_client = crate::algolia::Client::new(algolia_settings.clone())?;

        let db_pool = db::get_pool(
            settings
                .db_connect_options(settings::read_sql_proxy())
                .await?,
        )
        .await?;

        let algolia_manager = crate::algolia::Manager::new(algolia_settings, db_pool.clone())?;

        let guard = core::sentry::init(settings.sentry_api_key().await?.as_deref(), remote_target)?;

        (
            runtime_settings,
            s3,
            algolia_client,
            algolia_manager,
            db_pool,
            guard,
        )
    };

    // todo: find a better place for this...
    if let Some(algolia_manager) = algolia_manager {
        algolia_manager
            .migrate()
            .await
            .context("Algolia migration failed")?;

        let _ = algolia_manager.spawn();
    }

    let handle = thread::spawn(|| http::run(db_pool, runtime_settings, s3, algolia_client));

    log::info!("app started!");

    tokio::task::block_in_place(|| handle.join())
        .unwrap()
        .context("http server died")?;

    Ok(())
}
