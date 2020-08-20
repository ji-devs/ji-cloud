#[cfg(feature = "db")]
use sqlx::postgres::PgConnectOptions;

use crate::env::req_env;
use crate::google::{get_access_token_and_project_id, get_project_id, get_secret};
#[cfg(any(feature = "s3", feature = "db"))]
use config::RemoteTarget;

use jsonwebtoken::EncodingKey;

mod runtime;
pub use runtime::RuntimeSettings;

#[cfg(feature = "s3")]
mod s3;

#[cfg(feature = "s3")]
pub use s3::S3Settings;

#[cfg(feature = "jwk")]
mod jwk;

#[cfg(feature = "jwk")]
pub use jwk::JwkSettings;

pub struct SettingsManager {
    token: Option<String>,
    project_id: String,
}

#[cfg(all(feature = "sqlproxy", feature = "db"))]
const SQL_PROXY: bool = true;

#[cfg(all(not(feature = "sqlproxy"), feature = "db"))]
const SQL_PROXY: bool = false;

impl SettingsManager {
    async fn get_secret(&self, secret: &str) -> anyhow::Result<String> {
        match &self.token {
            Some(token) => get_secret(token, &self.project_id, secret).await,
            None => req_env(secret),
        }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let use_google_cloud = !crate::env::env_bool("DISABLE_GOOGLE_CLOUD");

        let (token, project_id) = if use_google_cloud {
            let (token, project_id) =
                get_access_token_and_project_id(crate::REMOTE_TARGET.google_credentials_env_name())
                    .await?;

            (Some(token), project_id)
        } else {
            let project_id = get_project_id(None)?;
            (None, project_id)
        };

        Ok(Self { token, project_id })
    }

    #[cfg(feature = "s3")]
    pub async fn s3_settings(&self) -> anyhow::Result<S3Settings> {
        S3Settings::new(crate::REMOTE_TARGET == RemoteTarget::Local)
    }

    #[cfg(feature = "jwk")]
    pub async fn jwk_settings(&self) -> anyhow::Result<JwkSettings> {
        JwkSettings::new(self.project_id.clone())
    }

    #[cfg(feature = "db")]
    pub async fn db_connect_options(&self) -> anyhow::Result<PgConnectOptions> {
        if crate::REMOTE_TARGET == RemoteTarget::Local && !SQL_PROXY {
            return Ok(crate::env::req_env("DATABASE_URL")?.parse::<PgConnectOptions>()?);
        }

        let db_pass = self.get_secret("DB_PASS").await?;

        let opts = PgConnectOptions::new()
            .username(config::REMOTE_DB_USER)
            .password(&db_pass)
            .database(config::REMOTE_DB_NAME);

        if SQL_PROXY {
            Ok(opts.host("localhost").port(config::SQL_PROXY_PORT))
        } else {
            let instance_connection =
                std::env::var("INSTANCE_CONNECTION_NAME").unwrap_or(match crate::REMOTE_TARGET {
                    RemoteTarget::Sandbox => config::DB_INSTANCE_SANDBOX.to_string(),
                    RemoteTarget::Release => config::DB_INSTANCE_RELEASE.to_string(),
                    _ => panic!("non-dev mode only makes sense for sandbox or release"),
                });

            let socket_path = std::env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

            Ok(opts.socket(format!("{}/{}", socket_path, instance_connection)))
        }
    }

    pub async fn runtime_settings(&self) -> anyhow::Result<RuntimeSettings> {
        let jwt_secret = self.get_secret("JWT_SECRET").await?;
        let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

        RuntimeSettings::new(jwt_encoding_key, jwt_secret)
    }
}
