use super::google::{get_access_token_and_project_id, get_secret};
use anyhow::Context;
use config::RemoteTarget;
use jsonwebtoken::EncodingKey;
use once_cell::sync::OnceCell;
use std::{
    env, fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub static SETTINGS: OnceCell<Settings> = OnceCell::new();

pub struct Settings {
    pub remote_target: RemoteTarget,
    pub local_insecure: bool,
    pub api_port: u16,
    pub pages_port: u16,
    pub epoch: Duration,
    pub jwt_encoding_key: EncodingKey,
    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    pub jwt_decoding_key: String,
    pub inter_server_secret: String,
    pub db_credentials: DbCredentials,
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
pub async fn init() -> anyhow::Result<()> {
    _init(RemoteTarget::Local, DbTarget::Proxy).await;
    Ok(())
}

#[cfg(all(feature = "local", not(feature = "sqlproxy")))]
pub async fn init() -> anyhow::Result<()> {
    init_local()
}

#[cfg(all(feature = "sandbox",))]
pub async fn init() -> anyhow::Result<()> {
    _init(
        RemoteTarget::Sandbox,
        DbTarget::Remote(RemoteTarget::Sandbox),
    )
    .await;
    Ok(())
}

#[cfg(all(feature = "release",))]
pub async fn init() -> anyhow::Result<()> {
    _init(
        RemoteTarget::Release,
        DbTarget::Remote(RemoteTarget::Release),
    )
    .await;
    Ok(())
}

fn req_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Missing required env var `{}`", key))
}

fn init_local() -> anyhow::Result<()> {
    let jwt_secret = req_env("JWT_SECRET")?;
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    let inter_server_secret = req_env("INTER_SERVER_SECRET")?;

    let db_credentials = DbCredentials::new_local()?;

    let settings = Settings::new(
        RemoteTarget::Local,
        db_credentials,
        jwt_encoding_key,
        jwt_secret,
        inter_server_secret,
    )?;

    SETTINGS
        .set(settings)
        .expect("SETTINGS can only be set once");

    Ok(())
}

async fn _init(remote_target: RemoteTarget, db_target: DbTarget) {
    let (token, project_id) =
        get_access_token_and_project_id(remote_target.google_credentials_env_name())
            .await
            .expect("couldn't get access token and project id!");

    let jwt_secret = get_secret(token.as_ref(), &project_id, "JWT_SECRET").await;
    let db_pass = get_secret(token.as_ref(), &project_id, "DB_PASS").await;
    let inter_server_secret = get_secret(token.as_ref(), &project_id, "INTER_SERVER").await;

    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal

    let db_credentials = DbCredentials::new(&db_pass, db_target);

    SETTINGS
        .set(
            Settings::new(
                remote_target,
                db_credentials,
                jwt_encoding_key,
                jwt_secret,
                inter_server_secret,
            )
            .expect("couldn't create settings"),
        )
        .expect("couldn't set settings");
}

impl fmt::Debug for Settings {
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

impl Settings {
    fn new(
        remote_target: RemoteTarget,
        db_credentials: DbCredentials,
        jwt_encoding_key: EncodingKey,
        jwt_decoding_key: String,
        inter_server_secret: String,
    ) -> anyhow::Result<Self> {
        let (api_port, pages_port) = match remote_target {
            RemoteTarget::Local => (
                req_env("LOCAL_API_PORT")?.parse()?,
                req_env("LOCAL_PAGES_PORT")?.parse()?,
            ),

            RemoteTarget::Sandbox | RemoteTarget::Release => (8080_u16, 8080_u16),
        };

        Ok(Self {
            remote_target,
            api_port,
            pages_port,
            local_insecure: remote_target == RemoteTarget::Local,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_credentials,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DbTarget {
    Local,
    Proxy,
    Remote(RemoteTarget),
}

#[derive(Debug)]
pub struct DbCredentials {
    pub dbname: String,
    pub user: String,
    pub pass: String,
    pub endpoint: DbEndpoint,
}
#[derive(Debug)]
pub enum DbEndpoint {
    Socket(String),
    Tcp(String, u16),
}

impl DbCredentials {
    pub fn new_local() -> anyhow::Result<Self> {
        //these are env vars since it depends on developer's local machine
        let user = req_env("LOCAL_DB_USER")?;
        let pass = req_env("LOCAL_DB_PASS")?;
        let port = req_env("LOCAL_DB_PORT")?
            .parse()
            .context("Port must be a u16")?;
        let dbname = req_env("LOCAL_DB_NAME")?;
        let host = "localhost".to_string();

        Ok(Self {
            user,
            pass,
            dbname,
            endpoint: DbEndpoint::Tcp(host, port),
        })
    }

    pub fn new(secret_db_pass: &str, db_target: DbTarget) -> Self {
        match db_target {
            DbTarget::Local => Self::new_local().unwrap(),
            DbTarget::Proxy => Self {
                user: config::REMOTE_DB_USER.to_string(),
                pass: secret_db_pass.to_string(),
                dbname: config::REMOTE_DB_NAME.to_string(),
                endpoint: DbEndpoint::Tcp("localhost".to_string(), config::SQL_PROXY_PORT),
            },
            DbTarget::Remote(remote_target) => {
                let instance_connection =
                    env::var("INSTANCE_CONNECTION_NAME").unwrap_or(match remote_target {
                        RemoteTarget::Sandbox => config::DB_INSTANCE_SANDBOX.to_string(),
                        RemoteTarget::Release => config::DB_INSTANCE_RELEASE.to_string(),
                        _ => panic!("non-dev mode only makes sense for sandbox or release"),
                    });

                let socket_path = env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

                Self {
                    user: config::REMOTE_DB_USER.to_string(),
                    pass: secret_db_pass.to_string(),
                    dbname: config::REMOTE_DB_NAME.to_string(),
                    endpoint: DbEndpoint::Socket(format!(
                        "{}/{}",
                        socket_path, instance_connection
                    )),
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match &self.endpoint {
            DbEndpoint::Tcp(host, port) => format!(
                "postgres:///{}?user={}&password={}&host={}&port={}",
                self.dbname, self.user, self.pass, host, port
            ),
            DbEndpoint::Socket(path) => format!(
                "postgres:///{}?user={}&password={}&host={}",
                self.dbname, self.user, self.pass, path
            ),
        }
    }
}
