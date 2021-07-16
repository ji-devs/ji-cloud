//! Media watch and transform service binary
//!
//! FIXME(BLOCKED): Right now is triggered by *ALL* upload events to Cloud Storage, including
//! processed media uploads to the final media bucket! Filtered in the application whether to
//! handle or not, which means 50%+ of trigger events/requests to this server are useless.
//!
//! Other events triggered include:
//! * cloud container deployments (rare on release, but can be frequent on sandbox)
//!
//! https://github.com/meteatamel/cloudrun-tutorial/issues/35
//!
//!

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

use actix_web::{post, web::Json};
use anyhow::Context;
use cloudevents::Event;
use config::JSON_BODY_LIMIT;
use core::{
    http::{get_addr, get_tcp_fd},
    settings::{self, RuntimeSettings, SettingsManager},
};
use ji_cloud_api::{
    db,
    google::storage,
    http::{bad_request_handler, Application},
    logger, s3,
    service::ServiceData,
};
use std::net::TcpListener;
use tokio::task;

// Only route is to accept POST /v1/media-watch.
// This application *shouldn't* need strict security requirements, as GCP ingress settings for this
// service allows only requests from authed GCP services. Will need to take this into account
// if this is changed in the future
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();

    logger::init()?;

    let (s3, gcs, runtime_settings) = {
        log::trace!("initializing settings and processes");
        let remote_target = settings::read_remote_target()?;

        let settings: SettingsManager = SettingsManager::new(remote_target).await?;

        let s3 = settings
            .s3_settings()
            .await?
            .map(s3::Client::new)
            .transpose()?;

        let gcs = settings
            .google_cloud_storage_settings()
            .await?
            .map(storage::Client::new)
            .transpose()?;

        let runtime_settings = settings.runtime_settings().await?;

        (s3, gcs, runtime_settings)
    };

    let handle = std::thread::spawn(|| build_and_run_media_watch(runtime_settings, s3, gcs));

    log::info!("media watch started!");

    tokio::task::block_in_place(|| handle.join())
        .unwrap()
        .context("media watch http server died")?;

    Ok(())
}

#[actix_web::main]
pub async fn build_and_run_media_watch(
    runtime_settings: RuntimeSettings,
    s3: Option<s3::Client>,
    gcs: Option<storage::Client>,
) -> anyhow::Result<()> {
    let app = build_media_watch(runtime_settings, s3, gcs)?;
    app.run_until_stopped().await?;

    Ok(())
}

fn build_media_watch(
    runtime_settings: RuntimeSettings,
    s3: Option<s3::Client>,
    gcs: Option<storage::Client>,
) -> anyhow::Result<Application> {
    let local_insecure = runtime_settings.is_local();
    let api_port = runtime_settings.api_port;

    let s3 = s3.map(ServiceData::new);
    let gcs = gcs.map(ServiceData::new);

    let server = actix_web::HttpServer::new(move || {
        let server = actix_web::App::new().data(runtime_settings.clone());

        let server = match s3.clone() {
            Some(s3) => server.app_data(s3),
            None => server,
        };

        let server = match gcs.clone() {
            Some(gcs) => server.app_data(gcs),
            None => server,
        };

        server
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .app_data(
                actix_web::web::JsonConfig::default()
                    .limit(JSON_BODY_LIMIT as usize)
                    .error_handler(|_, _| bad_request_handler()),
            )
            .app_data(
                actix_web::web::QueryConfig::default().error_handler(|_, _| bad_request_handler()),
            )
            .service(process_uploaded_media_trigger)
    });

    let listener = if let Some(l) = get_tcp_fd() {
        l
    } else {
        TcpListener::bind(get_addr(Some(api_port)))?
    };

    let port = listener.local_addr().unwrap().port();

    let server = server.listen(listener)?;

    Ok(Application::new(port, server.run()))
}

#[post("/v1/media-watch")]
async fn process_uploaded_media_trigger(
    gcs: ServiceData<storage::Client>,
    s3: ServiceData<s3::Client>,
    event: Event,
) -> Result<Json<()>, actix_web::Error> {
    log::info!("Received event {:?}", event);

    Ok(Json(()))
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let _ = dotenv::dotenv().ok();
//
//     logger::init()?;
//
//     let (s3, db_pool) = {
//         log::trace!("initializing settings and processes");
//         let remote_target = settings::read_remote_target()?;
//
//         let settings: SettingsManager = SettingsManager::new(remote_target).await?;
//
//         let s3 = settings
//             .s3_settings()
//             .await?
//             .map(s3::Client::new)
//             .transpose()?;
//
//         let db_pool = db::get_pool(
//             settings
//                 .db_connect_options(settings::read_sql_proxy())
//                 .await?,
//         )
//         .await?;
//
//         (s3, db_pool)
//     };
//
//     let s3 = s3.ok_or_else(|| anyhow::anyhow!("S3 client invalid"))?;
//
//     log::info!("task started!");
//
//     task::spawn({
//         let db_pool = db_pool.clone();
//         let s3 = s3.clone();
//         async move {
//             loop {
//                 let start = tokio::time::Instant::now();
//                 log::debug!("running watch_image loop");
//
//                 let delay_time =
//                     match ji_cloud_api::service::uploads::watch_image(&db_pool, &s3).await {
//                         // there was an image processed, delay for shorter.
//                         Ok(true) => tokio::time::Duration::from_secs(1),
//                         // Out of images to process, wait longer.
//                         Ok(false) => tokio::time::Duration::from_secs(5),
//                         Err(e) => {
//                             log::error!("watch_image task error: {:?}", e);
//
//                             continue;
//                         }
//                     };
//
//                 // only process an image at most every second (it probably takes longer than that to process one anyway)
//                 tokio::time::delay_until(start + delay_time).await;
//             }
//         }
//     });
//
//     task::spawn({
//         let db_pool = db_pool.clone();
//         let s3 = s3.clone();
//         async move {
//             loop {
//                 let start = tokio::time::Instant::now();
//                 log::debug!("running watch_animation loop");
//
//                 let delay_time =
//                     match ji_cloud_api::service::uploads::watch_animation(&db_pool, &s3).await {
//                         // there was an animation processed, delay for shorter.
//                         Ok(true) => tokio::time::Duration::from_secs(1),
//                         // Out of animations to process, wait longer.
//                         Ok(false) => tokio::time::Duration::from_secs(5),
//                         Err(e) => {
//                             log::error!("watch_animation task error: {:?}", e);
//
//                             continue;
//                         }
//                     };
//
//                 // only process an animation at most every second (it probably takes longer than that to process one anyway)
//                 tokio::time::delay_until(start + delay_time).await;
//             }
//         }
//     });
//
//     loop {
//         let start = tokio::time::Instant::now();
//         log::debug!("running watch_user_image loop");
//
//         let delay_time = match ji_cloud_api::service::uploads::watch_user_image(&db_pool, &s3).await
//         {
//             // there was an image processed, delay for shorter.
//             Ok(true) => tokio::time::Duration::from_secs(1),
//             // Out of images to process, wait longer.
//             Ok(false) => tokio::time::Duration::from_secs(5),
//             Err(e) => {
//                 log::error!("watch_user_image task error: {:?}", e);
//
//                 continue;
//             }
//         };
//
//         // only process an image at most every second (it probably takes longer than that to process one anyway)
//         tokio::time::delay_until(start + delay_time).await;
//     }
// }
