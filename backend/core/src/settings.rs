use anyhow::Context;
#[cfg(feature = "db")]
use sqlx::postgres::PgConnectOptions;

use crate::{
    env::{env_bool, keys, req_env},
    google::{get_access_token_and_project_id, get_optional_secret},
};
use config::RemoteTarget;
use std::str::FromStr;
use std::{
    convert::TryInto,
    env::VarError,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Reads a `RemoteTarget` from the arguments passed to the command.
pub fn read_remote_target() -> anyhow::Result<RemoteTarget> {
    let remote_target = match std::env::args().nth(1).as_deref() {
        Some("local") => RemoteTarget::Local,
        Some("sandbox") => RemoteTarget::Sandbox,
        Some("release") => RemoteTarget::Release,
        Some(s) => anyhow::bail!(
            "Unknown remote target: {} (expected local|sandbox|release)",
            s
        ),
        None => RemoteTarget::Local,
    };

    Ok(remote_target)
}

/// Reads weather or not sql_proxy should be used for database connections.
#[cfg(feature = "db")]
pub fn read_sql_proxy() -> bool {
    std::env::args().any(|s| s == "sqlproxy")
}

/// Settings related to Google's OAuth.
#[derive(Clone)]
pub struct GoogleOAuth {
    /// Client ID for google oauth.
    pub client: String,

    /// Client Secret for google oauth.
    pub secret: String,
}

impl GoogleOAuth {
    fn from_parts(client: Option<String>, secret: Option<String>) -> Option<Self> {
        Some(Self {
            client: client?,
            secret: secret?,
        })
    }
}

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

    /// Used to search for images via the bing image search api
    ///
    /// If missing, implies that bing searching is disabled.
    // todo: move this and make it runtime reloadable somehow (bing suggests rotating keys)
    pub bing_search_key: Option<String>,

    remote_target: RemoteTarget,

    /// Settings for google OAuth
    /// if missing / disabled, related routes will return `501 - Not Implemented`
    pub google_oauth: Option<GoogleOAuth>,

    /// Secret for signing/encrypting tokens.
    pub token_secret: Box<[u8; 32]>,

    /// How long *login* tokens are valid for (measured in seconds).
    /// * can only be set on `local`
    /// * optional, if missing it will use the server's compiled default (an indeterminate but reasonable amount of time)
    pub login_token_valid_duration: Option<chrono::Duration>,
}

impl RuntimeSettings {
    pub(crate) fn new(
        remote_target: RemoteTarget,
        bing_search_key: Option<String>,
        google_oauth: Option<GoogleOAuth>,
        token_secret: Box<[u8; 32]>,
        login_token_valid_duration: Option<chrono::Duration>,
    ) -> anyhow::Result<Self> {
        let (api_port, pages_port) = match remote_target {
            RemoteTarget::Local => (
                req_env("LOCAL_API_PORT")?.parse()?,
                req_env("LOCAL_PAGES_PORT")?.parse()?,
            ),

            RemoteTarget::Sandbox | RemoteTarget::Release => (8080_u16, 8080_u16),
        };

        let firebase_no_auth = env_bool("LOCAL_NO_FIREBASE_AUTH");

        assert_eq!(token_secret.len(), 32);

        Ok(Self {
            api_port,
            pages_port,
            firebase_no_auth,
            epoch: get_epoch(),
            remote_target,
            bing_search_key,
            google_oauth,
            token_secret,
            login_token_valid_duration,
        })
    }

    // shh, we're pretending not to be pulling this out of the aether
    /// The `RemoteTarget` that the settings are for.
    pub fn remote_target(&self) -> RemoteTarget {
        self.remote_target
    }

    /// Are we running "locally" (dev)?
    pub fn is_local(&self) -> bool {
        matches!(self.remote_target(), RemoteTarget::Local)
    }
}

/// Settings for initializing a S3 client
pub struct S3Settings {
    /// The s3 endpoint to connect to.
    pub endpoint: String,

    /// The s3 bucket that should be used for media.
    pub bucket: String,

    /// What's the access key's id?
    pub access_key_id: String,

    /// What's the access key's secret?
    pub secret_access_key: String,
}

/// Settings to initialize a algolia client.
#[derive(Clone, Debug)]
pub struct AlgoliaSettings {
    /// The AppID to provide to the algolia client.
    pub application_id: String,

    /// The key the backend uses for managing- indexing- `MEDIA_INDEX`.
    /// Needs the `addObject`, `deleteObject`, `settings`, and `editSettings` ACLs and access to `MEDIA_INDEX`.
    /// If [`None`], indexing will be disabled.
    pub management_key: Option<String>,

    /// The key that the backend uses for searching `MEDIA_INDEX`.
    /// Needs the `search` ACL with access to `MEDIA_INDEX`.
    /// If [`None`], searching will be disabled.
    pub backend_search_key: Option<String>,

    /// The index to use for operations on the algolia client.
    /// If [`None`], indexing and searching will be disabled.
    pub media_index: Option<String>,

    /// The key to use for the *frontend* for the algolia client.
    /// This key should be ratelimited, and restricted to a specific set of indecies (the media one- currently actually the "images" one) and any search suggestion indecies.
    pub frontend_search_key: Option<String>,
}

/// Manages access to settings.
pub struct SettingsManager {
    token: Option<String>,
    project_id: String,
    remote_target: RemoteTarget,
}

impl SettingsManager {
    async fn get_secret(&self, secret: &str) -> anyhow::Result<String> {
        self.get_optional_secret(secret)
            .await?
            .ok_or_else(|| anyhow::anyhow!("secret `{}` not present", secret))
    }

    async fn get_optional_secret(&self, secret: &str) -> anyhow::Result<Option<String>> {
        log::debug!("Getting secret `{}`", secret);
        match &self.token {
            // todo: this
            Some(token) => get_optional_secret(token, &self.project_id, secret)
                .await
                .with_context(|| anyhow::anyhow!("failed to get secret `{}`", secret)),
            None => match std::env::var(secret) {
                Ok(secret) => Ok(Some(secret)),
                Err(VarError::NotPresent) => Ok(None),
                Err(VarError::NotUnicode(_)) => {
                    Err(anyhow::anyhow!("secret `{}` wasn't unicode", secret))
                }
            },
        }
    }

    /// get a secret that may be optional, required, or in between (optional but warn on missing) depending on `self`'s configuration.
    async fn get_varying_secret(&self, secret: &str) -> anyhow::Result<Option<String>> {
        // currently, the only implemented functionality is "optional but warn on missing"
        let val = self.get_optional_secret(secret).await?;

        if val.is_none() {
            log::warn!(
                "Missing `{}` - related functionality will be disabled.",
                secret
            );
        }

        Ok(val)
    }

    // Sometimes unused due to some features not existing sometimes.
    #[allow(dead_code)]
    async fn get_secret_with_backup(
        &self,
        primary_key: &str,
        backup_key: &str,
    ) -> anyhow::Result<String> {
        if let Some(secret) = self.get_optional_secret(primary_key).await? {
            return Ok(secret);
        }

        let secret = self.get_secret(backup_key).await?;

        log::warn!(
            "Rename key `{}` to `{}` (loaded from backup var)",
            backup_key,
            primary_key,
        );

        Ok(secret)
    }

    /// Create a new instance of `Self`
    ///
    /// # Errors
    /// If it is decided that we need to use google cloud but fail, or we don't use google cloud and the `PROJECT_ID` env var is missing.
    pub async fn new(remote_target: RemoteTarget) -> anyhow::Result<Self> {
        let use_google_cloud = !crate::env::env_bool(keys::google::DISABLE);

        let (token, project_id) = if use_google_cloud {
            let (token, project_id) =
                get_access_token_and_project_id(remote_target.google_credentials_env_name())
                    .await?;

            (Some(token), project_id)
        } else {
            let project_id = req_env(keys::google::PROJECT_ID)?;
            (None, project_id)
        };

        Ok(Self {
            token,
            project_id,
            remote_target,
        })
    }

    /// Load the settings for s3.
    pub async fn s3_settings(&self) -> anyhow::Result<Option<S3Settings>> {
        let disable_local = crate::env::env_bool(keys::s3::DISABLE);

        if disable_local && self.remote_target == RemoteTarget::Local {
            return Ok(None);
        }

        let endpoint = match self.remote_target.s3_endpoint() {
            Some(endpoint) => Some(endpoint.to_string()),
            None => self.get_varying_secret(keys::s3::ENDPOINT).await?,
        };

        let bucket = match self.remote_target.s3_bucket() {
            Some(bucket) => Some(bucket.to_string()),
            None => self.get_varying_secret(keys::s3::BUCKET).await?,
        };

        let access_key_id = self.get_varying_secret(keys::s3::ACCESS_KEY).await?;

        let secret_access_key = self.get_varying_secret(keys::s3::SECRET).await?;

        match (endpoint, bucket, access_key_id, secret_access_key) {
            (Some(endpoint), Some(bucket), Some(access_key_id), Some(secret_access_key)) => {
                Ok(Some(S3Settings {
                    endpoint,
                    bucket,
                    access_key_id,
                    secret_access_key,
                }))
            }

            _ => return Ok(None),
        }
    }

    /// Load the key required for initializing sentry (for the api)
    pub async fn sentry_api_key(&self) -> anyhow::Result<Option<String>> {
        self.get_optional_secret(keys::SENTRY_DSN_API)
            .await
            .map(|it| it.filter(|it| !it.is_empty()))
    }

    /// Load the key required for initializing sentry (for pages)
    pub async fn sentry_pages_key(&self) -> anyhow::Result<Option<String>> {
        self.get_optional_secret(keys::SENTRY_DSN_PAGES)
            .await
            .map(|it| it.filter(|it| !it.is_empty()))
    }

    /// Load the settings for connecting to the db.
    #[cfg(feature = "db")]
    pub async fn db_connect_options(&self, sql_proxy: bool) -> anyhow::Result<PgConnectOptions> {
        if self.remote_target == RemoteTarget::Local && !sql_proxy {
            return Ok(crate::env::req_env(keys::db::DATABASE_URL)?.parse::<PgConnectOptions>()?);
        }

        let db_pass = self.get_secret(keys::db::PASSWORD).await?;

        let opts = PgConnectOptions::new()
            .username(config::REMOTE_DB_USER)
            .password(&db_pass)
            .database(config::REMOTE_DB_NAME);

        if sql_proxy {
            Ok(opts.host("localhost").port(config::SQL_PROXY_PORT))
        } else {
            let instance_connection = std::env::var(keys::db::INSTANCE_CONNECTION_NAME).unwrap_or(
                match self.remote_target {
                    RemoteTarget::Sandbox => config::DB_INSTANCE_SANDBOX.to_string(),
                    RemoteTarget::Release => config::DB_INSTANCE_RELEASE.to_string(),
                    _ => unreachable!(),
                },
            );

            let socket_path =
                std::env::var(keys::db::SOCKET_PATH).unwrap_or("/cloudsql".to_string());

            Ok(opts.socket(format!("{}/{}", socket_path, instance_connection)))
        }
    }

    /// Load the settings for Algolia.
    pub async fn algolia_settings(&self) -> anyhow::Result<Option<AlgoliaSettings>> {
        // Don't early return right away, notify of the other missing vars first.
        let application_id = self
            .get_varying_secret(keys::algolia::APPLICATION_ID)
            .await?;

        let media_index = self.get_varying_secret(keys::algolia::MEDIA_INDEX).await?;

        let management_key = self
            .get_varying_secret(keys::algolia::MANAGEMENT_KEY)
            .await?;

        let backend_search_key = self
            .get_varying_secret(keys::algolia::BACKEND_SEARCH_KEY)
            .await?;

        let frontend_search_key = self
            .get_varying_secret(keys::algolia::FRONTEND_SEARCH_KEY)
            .await?;

        // *now* returning is okay.
        let application_id = match application_id {
            Some(id) => id,
            None => return Ok(None),
        };

        Ok(Some(AlgoliaSettings {
            application_id,
            backend_search_key,
            management_key,
            media_index,
            frontend_search_key,
        }))
    }

    /// Load the `RuntimeSettings`.
    pub async fn runtime_settings(&self) -> anyhow::Result<RuntimeSettings> {
        let token_secret = self
            .get_secret(keys::TOKEN_SECRET)
            .await
            .and_then(|secret| {
                let secret = hex::decode(secret)?;

                let secret: [u8; 32] = secret.try_into().map_err(|s: Vec<u8>| {
                    anyhow::anyhow!(
                        "token secret must be 32 bytes long, it was: {} bytes long",
                        s.len()
                    )
                })?;

                Ok(Box::new(secret))
            })?;

        let bing_search_key = self.get_optional_secret(keys::BING_SEARCH_KEY).await?;

        let login_token_valid_duration = match self.remote_target {
            RemoteTarget::Local => self
                .get_optional_secret(keys::LOGIN_TOKEN_VALID_DURATION)
                .await?
                .as_deref()
                .map(i64::from_str)
                .transpose()?
                .map(chrono::Duration::seconds),
            _ => None,
        };

        let google_oauth = GoogleOAuth::from_parts(
            self.get_varying_secret(keys::GOOGLE_OAUTH_CLIENT).await?,
            self.get_varying_secret(keys::GOOGLE_OAUTH_SECRET).await?,
        );

        RuntimeSettings::new(
            self.remote_target,
            bing_search_key,
            google_oauth,
            token_secret,
            login_token_valid_duration,
        )
    }
}

fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
