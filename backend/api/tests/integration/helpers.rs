use chrono::{Duration, Utc};
use core::settings::{JwkAudiences, RuntimeSettings};
use ji_cloud_api::http::Application;
use rand::Rng;
use shared::config::RemoteTarget;
use sqlx::postgres::PgPoolOptions;
use sqlx::{postgres::PgConnectOptions, Executor, PgPool};

use crate::fixture::Fixture;
use crate::service::{Service, TestServicesSettings};

pub trait LoginExt {
    fn login(self) -> Self;
}

impl LoginExt for reqwest::RequestBuilder {
    fn login(self) -> Self {
        const SUB: &str = "Uv9rrKftNlHV0w2cbCHhf7wmtt5wQq8V";
        const CSRF: &str = "iQzmm4e8hVP6poK5";

        let key = &**PASETO_KEY;

        let token = ji_cloud_api::token::create_auth_token_no_cookie(
            key,
            Duration::minutes(10),
            SUB,
            CSRF.to_owned(),
            Utc::now(),
        )
        .expect("failed to create auth token");

        self.header("X-CSRF", CSRF)
            .header("Cookie", format!("X-AUTH={}", token))
        //.header("Authorization", format!("Bearer {}", token)) // PASSED
        //.query(&[(shared::domain::session::AUTH_QUERY_NAME, token.as_str())]) // PASSED
    }
}

#[must_use]
pub fn generate_paseto_key() -> [u8; 32] {
    let mut arr = [0; 32];
    rand::thread_rng().fill(&mut arr[..]);

    arr
}

pub static PASETO_KEY: once_cell::sync::Lazy<Box<[u8; 32]>> =
    once_cell::sync::Lazy::new(|| Box::new(generate_paseto_key()));

pub async fn initialize_server(
    fixtures: &[Fixture],
    services: &[Service],
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> Application {
    let (app, _) = initialize_server_and_get_db(fixtures, services, pool_opts, conn_opts).await;

    app
}

pub async fn test_initialize_server(
    fixtures: &[Fixture],
    services: &[Service],
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> Application {
    let (app, _) =
        test_initialize_server_and_get_db(fixtures, services, pool_opts, conn_opts).await;

    app
}

// FIXME: is there a cleaner way to get a db connection from the application?
pub async fn test_initialize_server_and_get_db(
    fixtures: &[Fixture],
    services: &[Service],
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> (Application, PgPool) {
    let _ = dotenv::dotenv().ok();

    log_init();
    let jwk_verifier = ji_cloud_api::jwk::create_verifier(JwkAudiences {
        oauth_client: "".to_string(),
        api: "".to_string(),
        media_watch: "".to_string(),
    });

    // Gets database url
    let db = ji_cloud_api::db::get_test_pool(pool_opts, conn_opts)
        .await
        .expect("failed to get db");

    for fixture in fixtures {
        db.execute(fixture.as_query())
            .await
            .expect("failed to execute fixture");
    }

    let (mail, s3, gcs, algolia) = match services.is_empty() {
        true => (None, None, None, None),
        false => {
            let settings = TestServicesSettings::new().await;
            match settings {
                Ok(s) => s.init_services(services).await,
                Err(e) => {
                    log::info!("Error while reading test service settings: {:?}", e);
                    (None, None, None, None)
                }
            }
        }
    };

    // todo: cache this.
    let settings = RuntimeSettings::new(
        RemoteTarget::Local,
        0,
        0,
        0,
        None,
        None,
        None,
        PASETO_KEY.clone(),
        None,
    );

    let app = ji_cloud_api::http::build(
        db.clone(),
        settings,
        s3,
        None, // TODO add test
        gcs,
        algolia,
        None,
        jwk_verifier,
        mail,
        None,
        None,
        None,
    )
    .expect("failed to initialize server");

    (app, db)
}

// FIXME: is there a cleaner way to get a db connection from the application?
pub async fn initialize_server_and_get_db(
    fixtures: &[Fixture],
    services: &[Service],
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> (Application, PgPool) {
    let _ = dotenv::dotenv().ok();

    log_init();
    let jwk_verifier = ji_cloud_api::jwk::create_verifier(JwkAudiences {
        oauth_client: "".to_string(),
        api: "".to_string(),
        media_watch: "".to_string(),
    });

    // Gets database url
    let db = ji_cloud_api::db::get_test_pool(pool_opts, conn_opts)
        .await
        .expect("failed to get db");

    for fixture in fixtures {
        db.execute(fixture.as_query())
            .await
            .expect("failed to execute fixture");
    }

    let (mail, s3, gcs, algolia) = match services.is_empty() {
        true => (None, None, None, None),
        false => {
            let settings = TestServicesSettings::new().await;
            match settings {
                Ok(s) => s.init_services(services).await,
                Err(e) => {
                    log::info!("Error while reading test service settings: {:?}", e);
                    (None, None, None, None)
                }
            }
        }
    };

    // todo: cache this.
    let settings = RuntimeSettings::new(
        RemoteTarget::Local,
        0,
        0,
        0,
        None,
        None,
        None,
        PASETO_KEY.clone(),
        None,
    );

    let app = ji_cloud_api::http::build(
        db.clone(),
        settings,
        s3,
        None, // TODO add test
        gcs,
        algolia,
        None,
        jwk_verifier,
        mail,
        None,
        None,
        None,
    )
    .expect("failed to initialize server");

    (app, db)
}

pub fn log_init() {
    let _ = env_logger::builder()
        .is_test(true)
        .parse_filters("info,sqlx::query=warn,sqlx::postgres::notice=warn")
        .parse_default_env()
        .try_init();
}
