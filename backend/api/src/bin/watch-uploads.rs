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

use core::settings::{self, SettingsManager};
use ji_cloud_api::{db, logger, s3};
use tokio::task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();

    logger::init()?;

    let (s3, db_pool) = {
        log::trace!("initializing settings and processes");
        let remote_target = settings::read_remote_target()?;

        let settings: SettingsManager = SettingsManager::new(remote_target).await?;

        let s3 = settings
            .s3_settings()
            .await?
            .map(s3::Client::new)
            .transpose()?;

        let db_pool = db::get_pool(
            settings
                .db_connect_options(settings::read_sql_proxy())
                .await?,
        )
        .await?;

        (s3, db_pool)
    };

    let s3 = s3.ok_or_else(|| anyhow::anyhow!("S3 client invalid"))?;

    log::info!("task started!");

    task::spawn({
        let db_pool = db_pool.clone();
        let s3 = s3.clone();
        async move {
            loop {
                let start = tokio::time::Instant::now();
                log::debug!("running watch_image loop");

                let delay_time =
                    match ji_cloud_api::service::uploads::watch_image(&db_pool, &s3).await {
                        // there was an image processed, delay for shorter.
                        Ok(true) => tokio::time::Duration::from_secs(1),
                        // Out of images to process, wait longer.
                        Ok(false) => tokio::time::Duration::from_secs(5),
                        Err(e) => {
                            log::error!("watch_image task error: {:?}", e);

                            continue;
                        }
                    };

                // only process an image at most every second (it probably takes longer than that to process one anyway)
                tokio::time::delay_until(start + delay_time).await;
            }
        }
    });

    task::spawn({
        let db_pool = db_pool.clone();
        let s3 = s3.clone();
        async move {
            loop {
                let start = tokio::time::Instant::now();
                log::debug!("running watch_animation loop");

                let delay_time =
                    match ji_cloud_api::service::uploads::watch_animation(&db_pool, &s3).await {
                        // there was an animation processed, delay for shorter.
                        Ok(true) => tokio::time::Duration::from_secs(1),
                        // Out of animations to process, wait longer.
                        Ok(false) => tokio::time::Duration::from_secs(5),
                        Err(e) => {
                            log::error!("watch_animation task error: {:?}", e);

                            continue;
                        }
                    };

                // only process an animation at most every second (it probably takes longer than that to process one anyway)
                tokio::time::delay_until(start + delay_time).await;
            }
        }
    });

    loop {
        let start = tokio::time::Instant::now();
        log::debug!("running watch_user_image loop");

        let delay_time = match ji_cloud_api::service::uploads::watch_user_image(&db_pool, &s3).await
        {
            // there was an image processed, delay for shorter.
            Ok(true) => tokio::time::Duration::from_secs(1),
            // Out of images to process, wait longer.
            Ok(false) => tokio::time::Duration::from_secs(5),
            Err(e) => {
                log::error!("watch_user_image task error: {:?}", e);

                continue;
            }
        };

        // only process an image at most every second (it probably takes longer than that to process one anyway)
        tokio::time::delay_until(start + delay_time).await;
    }
}
