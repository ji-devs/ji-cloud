#[cfg(feature = "db")]
use sqlx::postgres::PgConnectOptions;

use crate::{
    env::{env_bool, keys, req_env},
    google::{get_access_token_and_project_id, get_secret},
};
use config::RemoteTarget;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Settings that are accessed at runtime (as compared to startup time)
#[derive(Clone)]
pub struct RuntimeSettings {
    firebase_no_auth: bool,

    /// The port that the api runs on.
    pub api_port: u16,

    /// The code that the pages api runs on.
    pub pages_port: u16,

    /// When the server started.
    pub epoch: Duration,

    /// Used to encode jwt tokens.
    pub jwt_encoding_key: EncodingKey,

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    /// Used to _decode_ jwt tokens.
    jwt_decoding_key: String,
}

impl RuntimeSettings {
    pub(crate) fn new(
        jwt_encoding_key: EncodingKey,
        jwt_decoding_key: String,
    ) -> anyhow::Result<Self> {
        let (api_port, pages_port) = match crate::REMOTE_TARGET {
            RemoteTarget::Local => (
                req_env("LOCAL_API_PORT")?.parse()?,
                req_env("LOCAL_PAGES_PORT")?.parse()?,
            ),

            RemoteTarget::Sandbox | RemoteTarget::Release => (8080_u16, 8080_u16),
        };

        let firebase_no_auth = env_bool("LOCAL_NO_FIREBASE_AUTH");

        Ok(Self {
            api_port,
            pages_port,
            firebase_no_auth,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
        })
    }

    // shh, we're pretending not to be pulling this out of the aether
    /// The `RemoteTarget` that the settings are for.
    pub fn remote_target(&self) -> RemoteTarget {
        crate::REMOTE_TARGET
    }

    /// Are we running "locally" (dev)?
    pub fn is_local(&self) -> bool {
        matches!(self.remote_target(), RemoteTarget::Local)
    }

    /// Should we assume that anything that says firebase sent it is right?
    pub fn firebase_assume_valid(&self) -> bool {
        self.is_local() && self.firebase_no_auth
    }

    /// Get key used to decode jwt tokens.
    pub fn jwt_decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.jwt_decoding_key.as_bytes())
    }
}

/// Settings for initializing a S3 client
pub struct S3Settings {
    /// The s3 endpoint to connect to.
    pub endpoint: String,

    /// The s3 bucket that should be used for media.
    pub bucket: String,

    /// Should a s3 client be started? (for things like deleting images)
    pub use_client: bool,

    /// What's the access key's id?
    pub access_key_id: String,

    /// What's the access key's secret?
    pub secret_access_key: String,
}

/// Settings for managing JWKs from Google.
#[derive(Debug)]
pub struct JwkSettings {
    /// What audience should JWTs be checked for?
    pub audience: String,
    /// What issuer should JWTs be checked for?
    pub issuer: String,
}

/// Settings to initialize a algolia client.
pub struct AlgoliaSettings {
    /// The AppID to provide to the algolia client.
    pub application_id: String,
    /// The key to use for the algolia client.
    pub key: String,
    /// The index to use for operations on the algolia client.
    pub index: String,
}

/// Manages access to settings.
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

    // Sometimes unused due to some features not existing sometimes.
    #[allow(dead_code)]
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

    /// Create a new instance of `Self`
    ///
    /// # Errors
    /// If it is decided that we need to use google cloud but fail, or we don't use google cloud and the `PROJECT_ID` env var is missing.
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

    /// Load the settings for s3.
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

    /// Load the settings for JWKs.
    pub async fn jwk_settings(&self) -> anyhow::Result<JwkSettings> {
        let issuer = format!("{}/{}", config::JWK_ISSUER_URL, &self.project_id);

        Ok(JwkSettings {
            audience: self.project_id.clone(),
            issuer,
        })
    }

    /// Load the settings for connecting to the db.
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

    /// Load the settings for Algolia.
    pub async fn algolia_settings(&self) -> anyhow::Result<Option<AlgoliaSettings>> {
        let application_id = self
            .get_secret_with_backup(
                keys::algolia::APPLICATION_ID_NEW,
                keys::algolia::APPLICATION_ID_OLD,
            )
            .await?;

        let key = self.get_secret(keys::algolia::KEY).await?;

        let disable_local = env_bool(keys::algolia::DISABLE);
        if matches!(crate::REMOTE_TARGET, RemoteTarget::Local) && disable_local {
            return Ok(None);
        }

        let index = match crate::REMOTE_TARGET.algolia_image_index() {
            Some(it) => it.to_owned(),
            None => match self.get_secret(keys::algolia::IMAGE_INDEX).await {
                Ok(it) =>it,
                Err(_) => return Ok(None),
            }
        };

        Ok(Some(AlgoliaSettings {
            application_id,
            key,
            index,
        }))
    }

    /// Load the `RuntimeSettings`.
    pub async fn runtime_settings(&self) -> anyhow::Result<RuntimeSettings> {
        let jwt_secret = self.get_secret(keys::JWT_SECRET).await?;
        let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

        RuntimeSettings::new(jwt_encoding_key, jwt_secret)
    }
}

fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
