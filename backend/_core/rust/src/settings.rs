#[cfg(feature = "db")]
use sqlx::postgres::PgConnectOptions;

use crate::env::{keys, req_env};
use crate::google::{get_access_token_and_project_id, get_secret};
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

#[cfg(feature = "algolia")]
mod algolia;

#[cfg(feature = "algolia")]
pub use algolia::AlgoliaSettings;

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
        log::debug!("Getting secret `{}`", secret);
        match &self.token {
            Some(token) => get_secret(token, &self.project_id, secret).await,
            None => req_env(secret),
        }
    }

    async fn get_secret_with_backup(
        &self,
        primary_key: &str,
        backup_key: &str,
    ) -> anyhow::Result<String> {
        let err = match self.get_secret(primary_key).await {
            Ok(secret) => return Ok(secret),
            Err(e) => e,
        };

        match self.get_secret(backup_key).await {
            Ok(secret) => {
                log::warn!(
                    "Rename key `{}` to `{}` (loaded from backup var)",
                    backup_key,
                    primary_key,
                );
                Ok(secret)
            }

            // todo: don't discard the second error!
            Err(_) => Err(err),
        }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let use_google_cloud = !crate::env::env_bool(keys::google::DISABLE);

        let (token, project_id) = if use_google_cloud {
            let (token, project_id) =
                get_access_token_and_project_id(crate::REMOTE_TARGET.google_credentials_env_name())
                    .await?;

            (Some(token), project_id)
        } else {
            let project_id = req_env(keys::google::PROJECT_ID)?;
            (None, project_id)
        };

        Ok(Self { token, project_id })
    }

    #[cfg(feature = "s3")]
    pub async fn s3_settings(&self) -> anyhow::Result<S3Settings> {
        let endpoint = match crate::REMOTE_TARGET.s3_endpoint() {
            Some(e) => e.to_string(),
            None => self.get_secret(keys::s3::ENDPOINT).await?,
        };

        let bucket = match crate::REMOTE_TARGET.s3_bucket() {
            Some(b) => b.to_string(),
            None => self.get_secret(keys::s3::BUCKET).await?,
        };

        let access_key_id = self
            .get_secret_with_backup(keys::s3::ACCESS_KEY_NEW, keys::s3::ACCESS_KEY_OLD)
            .await?;
        let secret_access_key = self
            .get_secret_with_backup(keys::s3::SECRET_OLD, keys::s3::SECRET_NEW)
            .await?;

        let disable_local = crate::env::env_bool(keys::s3::DISABLE);

        Ok(S3Settings {
            endpoint,
            bucket,
            use_client: crate::REMOTE_TARGET != RemoteTarget::Local || !disable_local,
            access_key_id,
            secret_access_key,
        })
    }

    #[cfg(feature = "jwk")]
    pub async fn jwk_settings(&self) -> anyhow::Result<JwkSettings> {
        JwkSettings::new(self.project_id.clone())
    }

    #[cfg(feature = "db")]
    pub async fn db_connect_options(&self) -> anyhow::Result<PgConnectOptions> {
        if crate::REMOTE_TARGET == RemoteTarget::Local && !SQL_PROXY {
            return Ok(crate::env::req_env(keys::db::DATABASE_URL)?.parse::<PgConnectOptions>()?);
        }

        let db_pass = self.get_secret(keys::db::PASSWORD).await?;

        let opts = PgConnectOptions::new()
            .username(config::REMOTE_DB_USER)
            .password(&db_pass)
            .database(config::REMOTE_DB_NAME);

        if SQL_PROXY {
            Ok(opts.host("localhost").port(config::SQL_PROXY_PORT))
        } else {
            let instance_connection = std::env::var(keys::db::INSTANCE_CONNECTION_NAME).unwrap_or(
                match crate::REMOTE_TARGET {
                    RemoteTarget::Sandbox => config::DB_INSTANCE_SANDBOX.to_string(),
                    RemoteTarget::Release => config::DB_INSTANCE_RELEASE.to_string(),
                    _ => panic!("non-dev mode only makes sense for sandbox or release"),
                },
            );

            let socket_path =
                std::env::var(keys::db::SOCKET_PATH).unwrap_or("/cloudsql".to_string());

            Ok(opts.socket(format!("{}/{}", socket_path, instance_connection)))
        }
    }

    #[cfg(feature = "algolia")]
    pub async fn algolia_settings(&self) -> anyhow::Result<Option<AlgoliaSettings>> {
        let application_id = self
            .get_secret_with_backup(
                keys::algolia::APPLICATION_ID_NEW,
                keys::algolia::APPLICATION_ID_OLD,
            )
            .await?;

        let key = self.get_secret(keys::algolia::KEY).await?;

        Ok(AlgoliaSettings::new(application_id, key))
    }

    pub async fn runtime_settings(&self) -> anyhow::Result<RuntimeSettings> {
        let jwt_secret = self.get_secret(keys::JWT_SECRET).await?;
        let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

        RuntimeSettings::new(jwt_encoding_key, jwt_secret)
    }
}
