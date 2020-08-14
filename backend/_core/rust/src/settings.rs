#[cfg(any(not(feature = "local"), feature = "sqlproxy"))]
use super::google::{get_access_token_and_project_id, get_secret};
use config::RemoteTarget;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::postgres::PgConnectOptions;
use std::{
    env, fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct Settings {
    pub init: InitSettings,
    pub runtime: RuntimeSettings,
}

// Settings that are only useful to setup the program
pub struct InitSettings {
    pub jwk_audience: String,
    pub jwk_issuer: String,
    pub connect_options: PgConnectOptions,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_disable_local: bool,
}

impl InitSettings {
    pub fn new(project_id: String, connect_options: PgConnectOptions) -> anyhow::Result<Self> {
        let s3_endpoint = req_env("S3_ENDPOINT")?;
        let s3_bucket = req_env("S3_BUCKET")?;
        let s3_disable_local = env_bool("S3_LOCAL_DISABLE_CLIENT");

        let issuer = format!("{}/{}", config::JWK_ISSUER_URL, project_id);

        Ok(InitSettings {
            jwk_audience: project_id,
            jwk_issuer: issuer,
            connect_options,
            s3_endpoint,
            s3_bucket,
            s3_disable_local,
        })
    }
}

#[derive(Clone)]
pub struct RuntimeSettings {
    pub remote_target: RemoteTarget,
    firebase_no_auth: bool,
    pub api_port: u16,
    pub pages_port: u16,
    pub epoch: Duration,
    pub jwt_encoding_key: EncodingKey,
    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    jwt_decoding_key: String,
}

impl RuntimeSettings {
    fn new(
        remote_target: RemoteTarget,
        jwt_encoding_key: EncodingKey,
        jwt_decoding_key: String,
    ) -> anyhow::Result<Self> {
        let (api_port, pages_port) = match remote_target {
            RemoteTarget::Local => (
                req_env("LOCAL_API_PORT")?.parse()?,
                req_env("LOCAL_PAGES_PORT")?.parse()?,
            ),

            RemoteTarget::Sandbox | RemoteTarget::Release => (8080_u16, 8080_u16),
        };

        let firebase_no_auth = env_bool("LOCAL_NO_FIREBASE_AUTH");

        dbg!(std::mem::size_of::<Self>());

        Ok(Self {
            remote_target,
            api_port,
            pages_port,
            firebase_no_auth,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
        })
    }

    pub fn is_local(&self) -> bool {
        matches!(self.remote_target, RemoteTarget::Local)
    }

    pub fn firebase_assume_valid(&self) -> bool {
        self.is_local() && self.firebase_no_auth
    }

    pub fn jwt_decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.jwt_decoding_key.as_bytes())
    }
}

#[cfg(not(any(feature = "local", feature = "sandbox", feature = "release")))]
compile_error!("At least one of the `local`, `sandbox` or `release` features must be enabled.");

#[cfg(any(
    all(feature = "local", feature = "sandbox"),
    all(feature = "local", feature = "release"),
    all(feature = "sandbox", feature = "release"),
))]
compile_error!("Only one of `local`, `sandbox` or `release` features can be enabled.");

#[cfg(all(feature = "local", feature = "sqlproxy"))]
pub async fn init() -> anyhow::Result<Settings> {
    _init(RemoteTarget::Local, DbTarget::Proxy).await
}

#[cfg(all(feature = "local", not(feature = "sqlproxy")))]
pub async fn init() -> anyhow::Result<Settings> {
    let jwt_secret = req_env("JWT_SECRET")?;
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
    let project_id = env::var("PROJECT_ID").unwrap_or_default();

    Ok(Settings {
        runtime: RuntimeSettings::new(RemoteTarget::Local, jwt_encoding_key, jwt_secret)?,
        init: InitSettings::new(
            project_id,
            req_env("DATABASE_URL")?.parse::<PgConnectOptions>()?,
        )?,
    })
}

#[cfg(feature = "sandbox")]
pub async fn init() -> anyhow::Result<Settings> {
    _init(
        RemoteTarget::Sandbox,
        DbTarget::Remote(RemoteTarget::Sandbox),
    )
    .await
}

#[cfg(feature = "release")]
pub async fn init() -> anyhow::Result<Settings> {
    _init(
        RemoteTarget::Release,
        DbTarget::Remote(RemoteTarget::Release),
    )
    .await
}

pub fn req_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Missing required env var `{}`", key))
}

pub fn env_bool(key: &str) -> bool {
    env::var(key).map_or(false, |it| ["true", "1", "y"].contains(&it.as_ref()))
}

#[cfg(any(not(feature = "local"), feature = "sqlproxy"))]
async fn _init(remote_target: RemoteTarget, db_target: DbTarget) -> anyhow::Result<Settings> {
    let (token, project_id) =
        get_access_token_and_project_id(remote_target.google_credentials_env_name())
            .await
            .expect("couldn't get access token and project id!");

    let jwt_secret = get_secret(token.as_ref(), &project_id, "JWT_SECRET").await;
    let db_pass = get_secret(token.as_ref(), &project_id, "DB_PASS").await;

    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal

    Ok(Settings {
        init: InitSettings::new(project_id, db_target.into_connect_options(&db_pass))?,
        runtime: RuntimeSettings::new(remote_target, jwt_encoding_key, jwt_secret)?,
    })
}

impl fmt::Debug for RuntimeSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // when finish_non_exaustive is stable, use that.
        f.debug_struct("Settings")
            .field("remote_target", &self.remote_target)
            .field("api_port", &self.api_port)
            .field("pages_port", &self.pages_port)
            .finish()
    }
}

fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg(any(not(feature = "local"), feature = "sqlproxy"))]
enum DbTarget {
    Proxy,
    Remote(RemoteTarget),
}

#[cfg(any(not(feature = "local"), feature = "sqlproxy"))]
impl DbTarget {
    fn into_connect_options(self, secret_db_pass: &str) -> PgConnectOptions {
        // Proxy target + remote target
        let base = PgConnectOptions::new()
            .username(config::REMOTE_DB_USER)
            .password(secret_db_pass)
            .database(config::REMOTE_DB_NAME);

        match self {
            DbTarget::Proxy => base.host("localhost").port(config::SQL_PROXY_PORT),

            DbTarget::Remote(remote_target) => {
                let instance_connection =
                    env::var("INSTANCE_CONNECTION_NAME").unwrap_or(match remote_target {
                        RemoteTarget::Sandbox => config::DB_INSTANCE_SANDBOX.to_string(),
                        RemoteTarget::Release => config::DB_INSTANCE_RELEASE.to_string(),
                        _ => panic!("non-dev mode only makes sense for sandbox or release"),
                    });

                let socket_path = env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

                base.socket(format!("{}/{}", socket_path, instance_connection))
            }
        }
    }
}
