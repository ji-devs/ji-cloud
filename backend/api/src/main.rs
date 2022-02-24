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

use std::thread;

use anyhow::Context;
use core::settings::{self, SettingsManager};
use sentry_tracing::EventFilter;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, EnvFilter, Registry};

use ji_cloud_api::{algolia, db, http, jwk, logger, service, translate};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();

    logger::init()?;

    let (
        runtime_settings,
        s3,
        gcp_key_store,
        gcs,
        algolia_client,
        algolia_key_store,
        algolia_manager,
        db_pool,
        jwk_verifier,
        mail_client,
        media_upload_cleaner,
        google_translate,
        _guard,
    ) = {
        log::trace!("initializing settings and processes");
        let remote_target = settings::read_remote_target()?;
        let settings: SettingsManager = SettingsManager::new(remote_target).await?;

        // `guard` needs to remain in scope so that we don't lose our Sentry config.
        let guard = core::sentry::init(settings.sentry_api_key().await?.as_deref(), remote_target)?;

        // Sentry is weird. By default info! and below events are used as breadcrumbs. I don't want
        // this, I want insight into the actual functioning of the system, not just errors.
        let sentry_layer = sentry_tracing::layer()
            .event_filter(|_md| EventFilter::Event)
            .span_filter(|_md| true);

        let fmt_layer = tracing_subscriber::fmt::layer().with_span_events(FmtSpan::CLOSE);

        // Use the RUST_LOG= environment variable to set which minimum trace level to use.
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        let tracing_subscriber = Registry::default()
            .with(env_filter)
            .with(sentry_layer)
            .with(fmt_layer);

        tracing::subscriber::set_global_default(tracing_subscriber)
            .expect("Unable to set global subscriber");

        let runtime_settings = settings.runtime_settings().await?;

        let s3 = settings
            .s3_settings()
            .await?
            .map(service::s3::Client::new)
            .transpose()?;

        let gcp_key_store = settings
            .google_cloud_serivce_token()
            .await?
            .map(service::GcpAccessKeyStore::new)
            .transpose()?;

        let gcs = settings
            .google_cloud_storage_settings()
            .await?
            .map(service::storage::Client::new)
            .transpose()?;

        let algolia_settings = settings.algolia_settings().await?;

        let algolia_client = crate::algolia::Client::new(algolia_settings.clone())?;

        let algolia_key_store = algolia_settings
            .as_ref()
            .and_then(|it| it.frontend_search_key.clone())
            .map(crate::algolia::SearchKeyStore::new)
            .transpose()?;

        let db_pool = db::get_pool(
            settings
                .db_connect_options(settings::read_sql_proxy())
                .await?,
        )
        .await?;

        let algolia_manager = crate::algolia::Manager::new(algolia_settings, db_pool.clone())?;

        let media_upload_cleaner =
            service::upload::cleaner::UploadCleaner::new(db_pool.clone(), db::UPLOADS_DB_SCHEMA)?;

        let google_translate = translate::GoogleTranslate::new(db_pool.clone(), &runtime_settings)?;

        let jwk_verifier =
            jwk::create_verifier(settings.jwk_audience_settings(&runtime_settings).await?);

        let _ = jwk::run_task(jwk_verifier.clone());

        let mail_client = settings
            .email_client_settings()
            .await?
            .map(service::mail::Client::new);

        (
            runtime_settings,
            s3,
            gcp_key_store,
            gcs,
            algolia_client,
            algolia_key_store,
            algolia_manager,
            db_pool,
            jwk_verifier,
            mail_client,
            media_upload_cleaner,
            google_translate,
            guard,
        )
    };

    let handle = thread::spawn(|| {
        http::build_and_run(
            db_pool,
            runtime_settings,
            s3,
            gcp_key_store,
            gcs,
            algolia_client,
            algolia_key_store,
            jwk_verifier,
            mail_client,
            algolia_manager,
            media_upload_cleaner,
            google_translate,
        )
    });

    log::info!("app started!");

    tokio::task::block_in_place(|| handle.join())
        .unwrap()
        .context("http server died")?;

    Ok(())
}
