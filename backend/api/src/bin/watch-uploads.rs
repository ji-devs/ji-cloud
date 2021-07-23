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

use std::net::TcpListener;

use actix_web::{
    post,
    web::{Data, Json, Query},
};
use anyhow::Context;
use cloudevents::Event;
use config::JSON_BODY_LIMIT;
use core::{
    http::{get_addr, get_tcp_fd},
    settings::{self, RuntimeSettings, SettingsManager},
};
use ji_cloud_api::{
    db, error,
    http::{bad_request_handler, Application},
    logger, s3,
    service::{
        event_arc::{self, audit_log, EventResource, EventSource},
        notifications, uploads, ServiceData,
    },
};
use shared::media::{FileKind, MediaLibrary, PngImageFile};
use sqlx::PgPool;
use std::convert::TryFrom;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();

    logger::init()?;

    let (s3, event_arc, notifications, db_pool, runtime_settings) = {
        log::trace!("initializing settings and processes");
        let remote_target = settings::read_remote_target()?;

        let settings: SettingsManager = SettingsManager::new(remote_target).await?;

        let db_pool = db::get_pool(
            settings
                .db_connect_options(settings::read_sql_proxy())
                .await?,
        )
        .await?;

        let s3 = settings
            .s3_settings()
            .await?
            .map(s3::Client::new)
            .transpose()?;

        let event_arc = settings
            .google_cloud_eventarc_settings()
            .await?
            .map(event_arc::Client::new)
            .transpose()?;

        let notifications = settings
            .fcm_settings()
            .await?
            .map(notifications::Client::new)
            .transpose()?;

        let runtime_settings = settings.runtime_settings().await?;

        (s3, event_arc, notifications, db_pool, runtime_settings)
    };

    let handle = std::thread::spawn(|| {
        build_and_run_media_watch(db_pool, runtime_settings, s3, event_arc, notifications)
    });

    log::info!("media watch started!");

    tokio::task::block_in_place(|| handle.join())
        .unwrap()
        .context("media watch http server died")?;

    Ok(())
}

#[actix_web::main]
pub async fn build_and_run_media_watch(
    db_pool: PgPool,
    runtime_settings: RuntimeSettings,
    s3: Option<s3::Client>,
    event_arc: Option<event_arc::Client>,
    notifications: Option<notifications::Client>,
) -> anyhow::Result<()> {
    let app = build_media_watch(runtime_settings, db_pool, s3, event_arc, notifications)?;
    app.run_until_stopped().await?;

    Ok(())
}

fn build_media_watch(
    runtime_settings: RuntimeSettings,
    db_pool: PgPool,
    s3: Option<s3::Client>,
    event_arc: Option<event_arc::Client>,
    notifications: Option<notifications::Client>,
) -> anyhow::Result<Application> {
    // let local_insecure = runtime_settings.is_local();
    let media_watch_port = runtime_settings.media_watch_port;

    let s3 = s3.map(ServiceData::new);
    let event_arc = event_arc.map(ServiceData::new);
    let notifications = notifications.map(ServiceData::new);

    // This application *shouldn't* need strict security requirements, as GCP ingress settings for this
    // service allows only requests from authed GCP services. Will need to take this into account
    // if this is changed in the future
    let server = actix_web::HttpServer::new(move || {
        let server = actix_web::App::new()
            .data(db_pool.clone())
            .data(runtime_settings.clone());

        let server = match s3.clone() {
            Some(s3) => server.app_data(s3),
            None => server,
        };

        let server = match event_arc.clone() {
            Some(event_arc) => server.app_data(event_arc),
            None => server,
        };

        let server = match notifications.clone() {
            Some(notifications) => server.app_data(notifications),
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
        TcpListener::bind(get_addr(Some(media_watch_port)))?
    };

    let port = listener.local_addr().unwrap().port();

    let server = server.listen(listener)?;

    Ok(Application::new(port, server.run()))
}

/// Only route is to accept for this application is POST /v1/media-watch. Checks that it is the
/// upload to processing event that we are looking for.
/// https://cloud.google.com/eventarc/docs/cloudevents#cloud-audit-logs
#[post("/v1/media-watch")]
async fn process_uploaded_media_trigger(
    db: Data<PgPool>,
    s3: ServiceData<s3::Client>,
    fcm: ServiceData<notifications::Client>,
    event_arc: ServiceData<event_arc::Client>,
    event: Event,
    _query: Option<Query<audit_log::Query>>,
) -> Result<Json<()>, error::EventArc> {
    type Error = error::EventArc;

    // if let Some(cloud_events_mode) = query.into_inner().cloud_events_mode {
    //     if cloud_events_mode != "__CE_PUBSUB_BINDING" {
    //         return Err(Error::InvalidEventSource);
    //     }
    // } else {
    //     return Err(Error::InvalidEventSource);
    // }

    let event: audit_log::Event = audit_log::Event::try_from(event)?;

    let event_source: EventSource = EventSource::from_str(&event.source)?;
    if event_source.service_name != event_arc.storage_service_name()
        || event_source.project_id != event_arc.project_id()
    {
        log::warn!("Bad event source: {:?}", event_source);
        return Err(Error::InvalidEventSource);
    }

    let event_data: audit_log::Data = event.try_decode_event_payload()?;
    if event_data.resource.labels.bucket_name != s3.processing_bucket() {
        log::warn!("Bad event data: {:?}", event_data);
        return Err(Error::InvalidEventSource);
    }

    let event_resource: EventResource =
        EventResource::from_str(&event_data.proto_payload.resource_name)?;

    // TODO: use gcs instead of S3
    let res = match event_resource.file_kind {
        FileKind::ImagePng(PngImageFile::Original) => match event_resource.library {
            MediaLibrary::Global => uploads::process_image(&db, &s3, event_resource.id)
                .await
                .map_err(|_| Error::NotProcessed)?,
            MediaLibrary::User => uploads::process_user_image(&db, &s3, event_resource.id)
                .await
                .map_err(|_| Error::NotProcessed)?,
            _ => return Err(Error::InvalidEventResource),
        },
        FileKind::AnimationGif => uploads::process_animation(&db, &s3, event_resource.id)
            .await
            .map_err(|_| Error::NotProcessed)?,
        _ => return Err(Error::InvalidEventResource),
    };

    if res == true {
        log::info!("Finalizing upload...");

        uploads::finalize_upload(
            &fcm,
            event_resource.library,
            event_resource.id,
            event_resource.file_kind,
        )
        .await?;
        Ok(Json(()))
    } else {
        Err(Error::NotProcessed)
    }
}
