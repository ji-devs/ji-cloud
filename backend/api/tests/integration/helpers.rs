use std::{collections::HashSet, sync::Mutex};

use chrono::{Duration, Utc};
use core::settings::{JwkAudiences, RuntimeSettings};
use ji_cloud_api::http::Application;
use rand::Rng;
use shared::config::RemoteTarget;
use sqlx::{Connection, Executor, PgPool, Pool, Postgres};

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
fn generate_db_name() -> String {
    let uniform = rand::distributions::Uniform::new_inclusive('a', 'z');

    rand::thread_rng().sample_iter(uniform).take(32).collect()
}

#[must_use]
pub fn generate_paseto_key() -> [u8; 32] {
    let mut arr = [0; 32];
    rand::thread_rng().fill(&mut arr[..]);

    arr
}

pub struct DbManager {
    base: String,
    get_url: fn(&str, &str) -> String,
    names: Mutex<HashSet<String>>,
}

impl DbManager {
    fn new(base: String, get_url: fn(&str, &str) -> String) -> Self {
        Self {
            base,
            get_url,
            names: Mutex::new(HashSet::new()),
        }
    }

    pub fn get_url(&self, name: &str) -> String {
        (self.get_url)(&self.base, name)
    }

    // todo: have a drop guard for this? (trying to prevent leaks)
    pub fn allocate_name(&self) -> String {
        let mut names = self.names.lock().expect("names poisoned");
        loop {
            let name = generate_db_name();
            if !names.insert(name.clone()) {
                continue;
            }

            return name;
        }
    }

    #[allow(unused)]
    pub fn deallocate_name(&self, name: &str) {
        self.names.lock().expect("names poisoned").remove(name);
    }

    pub async fn create(&self) -> anyhow::Result<String> {
        // todo: cache this
        let mut conn = sqlx::PgConnection::connect(&self.base).await?;

        let name = self.allocate_name();
        sqlx::query(&format!(r#"create database "{}""#, name))
            .execute(&mut conn)
            .await?;

        Ok(name)
    }
}

pub fn init_db() -> DbManager {
    if let Some(base) = std::env::var("DATABASE_URL").ok() {
        DbManager::new(base, |base, name| {
            let mut base = base.to_owned();
            base.push('/');
            base.push_str(name);
            base
        })
    } else {
        let pg_tmp = std::env::var("PG_TMP")
            .ok()
            .unwrap_or("../script/ephemeralpg/pg_tmp.sh".to_owned());

        let output = std::process::Command::new(pg_tmp)
            .output()
            .expect("Failed to get output from pg_tmp");

        let base = std::str::from_utf8(&output.stdout)
            .expect("pg_tmp didn't output UTF-8")
            .trim()
            .to_owned();

        DbManager::new(base, |base, name| base.replace("test", name))
    }

    // // use a single key for the entire instance (they take time to generate)
    // t.context.pasetoKey = (await paseto.V2.generateKey('local'));

    // // this gets used in every server, cache it.
    // t.context.pasetoKeyHex = t.context.pasetoKey.export().toString('hex');
}

static DB_URL_MANAGER: once_cell::sync::Lazy<DbManager> = once_cell::sync::Lazy::new(init_db);

pub static PASETO_KEY: once_cell::sync::Lazy<Box<[u8; 32]>> =
    once_cell::sync::Lazy::new(|| Box::new(generate_paseto_key()));

pub async fn initialize_server(
    fixtures: &[Fixture],
    services: &[Service],
    pool: Pool<Postgres>,
) -> Application {
    let (app, _) = initialize_server_and_get_db(fixtures, services, pool).await;
    app
}

// FIXME: is there a cleaner way to get a db connection from the application?
pub async fn initialize_server_and_get_db(
    fixtures: &[Fixture],
    services: &[Service],
    pool: Pool<Postgres>,
) -> (Application, PgPool) {
    let _ = dotenv::dotenv().ok();

    log_init();
    let jwk_verifier = ji_cloud_api::jwk::create_verifier(JwkAudiences {
        oauth_client: "".to_string(),
        api: "".to_string(),
        media_watch: "".to_string(),
    });

    println!("pool: {:?}", pool);

    let db_name = DB_URL_MANAGER.create().await.expect("failed to create db");

    let db_url = DB_URL_MANAGER.get_url(&db_name);

    let db = ji_cloud_api::db::get_test_pool(db_url.parse().expect("db url was invalid"), pool)
        .await
        .expect("failed to get db");

    // let db = pool.clone();

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
