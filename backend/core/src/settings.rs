#[cfg(feature = "db")]
use sqlx::postgres::PgConnectOptions;

#[allow(unused_imports)]
use crate::config::{
    DB_INSTANCE_RELEASE, DB_INSTANCE_SANDBOX, EVENTARC_AUDITLOG_SERVICE_NAME, REMOTE_DB_NAME,
    REMOTE_DB_USER, SQL_PROXY_PORT,
};

use crate::{
    env::{keys, req_env},
    google::{
        get_access_token_response_and_project_id, get_optional_secret, GoogleAccessTokenResponse,
    },
};

use anyhow::Context;
use chrono::{DateTime, Utc};
use shared::config::RemoteTarget;
use std::{
    convert::TryInto,
    env::VarError,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
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
    /// The port that the api runs on.
    pub api_port: u16,

    /// The code that the pages api runs on.
    pub pages_port: u16,

    /// The port that the media transformation api runs on.
    pub media_watch_port: u16,

    /// When the server started.
    pub epoch: std::time::Duration,

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
    /// create new runtime settings.
    pub fn new(
        remote_target: RemoteTarget,
        api_port: u16,
        pages_port: u16,
        media_watch_port: u16,
        bing_search_key: Option<String>,
        google_oauth: Option<GoogleOAuth>,
        token_secret: Box<[u8; 32]>,
        login_token_valid_duration: Option<chrono::Duration>,
    ) -> Self {
        assert_eq!(token_secret.len(), 32);

        Self {
            api_port,
            pages_port,
            media_watch_port,
            epoch: get_epoch(),
            remote_target,
            bing_search_key,
            google_oauth,
            token_secret,
            login_token_valid_duration,
        }
    }

    pub(crate) fn with_env(
        remote_target: RemoteTarget,
        bing_search_key: Option<String>,
        google_oauth: Option<GoogleOAuth>,
        token_secret: Box<[u8; 32]>,
        login_token_valid_duration: Option<chrono::Duration>,
    ) -> anyhow::Result<Self> {
        let (api_port, pages_port, media_watch_port) = match remote_target {
            RemoteTarget::Local => (
                req_env("LOCAL_API_PORT")?.parse()?,
                req_env("LOCAL_PAGES_PORT")?.parse()?,
                req_env("LOCAL_MEDIA_TRANSFORM_PORT")?.parse()?,
            ),

            RemoteTarget::Sandbox | RemoteTarget::Release => (8080_u16, 8080_u16, 8080_u16),
        };

        assert_eq!(token_secret.len(), 32);

        Ok(Self {
            api_port,
            pages_port,
            media_watch_port,
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
    pub media_bucket: String,

    /// The s3 bucket that should be used for media.
    pub processing_bucket: String,

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

    /// The index to use for operations relating to jigs on the algolia client.
    /// If [`None`], indexing and searching will be disabled.
    pub jig_index: Option<String>,

    /// The key to use for the *frontend* for the algolia client.
    /// This key should be ratelimited, and restricted to a specific set of indecies (the media one- currently actually the "images" one) and any search suggestion indecies.
    pub frontend_search_key: Option<String>,
}

/// Settings for the email (sendgrid) client.
#[derive(Clone, Debug)]
pub struct EmailClientSettings {
    /// Sendgrid / email client api key.
    // Is optional. If missing, all mailing services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub api_key: String,

    /// Email client sender email address.
    /// Is optional. If missing, all mailing services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub sender_email: String,

    /// Email client template ID for verifying emails at signup.
    /// Is optional. If missing, email verification (at signup) will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub signup_verify_template: Option<String>,

    /// Email client template ID for resetting passwords.
    /// Is optional. If missing, password resetting will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub password_reset_template: Option<String>,

    /// Email client template ID for resetting emails.
    /// Is optional. If missing, email resetting will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub email_reset_template: Option<String>,
}

// TODO: unify google services clients' auth tokens and project_id requirements

/// Settings for the Google Cloud Storage client
#[derive(Clone, Debug)]
pub struct GoogleCloudStorageSettings {
    /// Bucket for processed media.
    pub media_bucket: String,

    /// Bucket for raw media uploads.
    pub processing_bucket: String,
}

/// Settings for handling Google EventArc triggers
#[derive(Clone, Debug)]
pub struct GoogleCloudEventArcSettings {
    /// Service name for Google Cloud storage
    pub storage_service_name: String,

    /// ID of the GCP project
    pub project_id: String,

    /// Topic name for raw media upload event
    pub media_uploaded_topic: String,

    /// Topic name for completed media processing event
    pub media_processed_topic: String,
}

/// Firebase client, for Firestore
#[derive(Clone, Debug)]
pub struct FirebaseSettings {
    /// ID of the GCP project
    pub project_id: String,
}

/// Sets the possible `aud` targets for GCP authenticated account requests. This can be for users (through
/// OAuth2) or for GCP services invoking the Cloud Run instances directly by setting the `aud` target.
///
/// See:
/// * https://cloud.google.com/run/docs/authenticating/service-to-service
/// * https://stackoverflow.com/questions/58683365/google-cloud-run-authentication-service-to-service
#[derive(Debug)]
pub struct JwkAudiences {
    /// OAuth user client ID
    pub oauth_client: String,
    /// URL to API instance on Cloud Run
    pub api: String,
    /// URL to media-watch instance on Cloud Run
    pub media_watch: String,
}

/// Manages access to settings.
pub struct SettingsManager {
    token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
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
    //
    // fn get_secret_from_env(&self, secret: &str) -> anyhow::Result<Option<String>> {
    //     match std::env::var(secret) {
    //         Ok(secret) => Ok(Some(secret)),
    //         Err(VarError::NotPresent) => Ok(None),
    //         Err(VarError::NotUnicode(_)) => {
    //             Err(anyhow::anyhow!("secret `{}` wasn't unicode", secret))
    //         }
    //     }
    // }

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

        let (token, expires_at, project_id) = if use_google_cloud {
            let (token_response, project_id) = get_access_token_response_and_project_id(
                remote_target.google_credentials_env_name(),
            )
            .await?;

            (
                token_response.access_token,
                token_response.expires_at,
                project_id,
            )
        } else {
            let project_id = req_env(keys::google::PROJECT_ID)?;
            (None, None, project_id)
        };

        Ok(Self {
            token,
            expires_at,
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

        let media_bucket = match self.remote_target.s3_bucket() {
            Some(bucket) => Some(bucket.to_string()),
            None => self.get_varying_secret(keys::s3::MEDIA_BUCKET).await?,
        };

        let processing_bucket = match self.remote_target.s3_processing_bucket() {
            Some(bucket) => Some(bucket.to_string()),
            None => self.get_varying_secret(keys::s3::PROCESSING_BUCKET).await?,
        };

        let access_key_id = self.get_varying_secret(keys::s3::ACCESS_KEY).await?;

        let secret_access_key = self.get_varying_secret(keys::s3::SECRET).await?;

        match (
            endpoint,
            media_bucket,
            processing_bucket,
            access_key_id,
            secret_access_key,
        ) {
            (
                Some(endpoint),
                Some(media_bucket),
                Some(processing_bucket),
                Some(access_key_id),
                Some(secret_access_key),
            ) => Ok(Some(S3Settings {
                endpoint,
                media_bucket,
                processing_bucket,
                access_key_id,
                secret_access_key,
            })),

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
            .username(REMOTE_DB_USER)
            .password(&db_pass)
            .database(REMOTE_DB_NAME);

        if sql_proxy {
            Ok(opts.host("localhost").port(SQL_PROXY_PORT))
        } else {
            let instance_connection = std::env::var(keys::db::INSTANCE_CONNECTION_NAME).unwrap_or(
                match self.remote_target {
                    RemoteTarget::Sandbox => DB_INSTANCE_SANDBOX.to_string(),
                    RemoteTarget::Release => DB_INSTANCE_RELEASE.to_string(),
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

        let jig_index = self.get_varying_secret(keys::algolia::JIG_INDEX).await?;

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
            jig_index,
            frontend_search_key,
        }))
    }

    /// Load the Email Client settings.
    pub async fn email_client_settings(&self) -> anyhow::Result<Option<EmailClientSettings>> {
        let disable_local = crate::env::env_bool(keys::email::DISABLE);

        if disable_local && self.remote_target == RemoteTarget::Local {
            return Ok(None);
        }

        let api_key = self.get_varying_secret(keys::email::API_KEY).await?;

        let sender_email = self.get_varying_secret(keys::email::SENDER_EMAIL).await?;

        let signup_verify_template = self
            .get_varying_secret(keys::email::SIGNUP_VERIFY_TEMPLATE)
            .await?;

        let password_reset_template = self
            .get_varying_secret(keys::email::PASSWORD_RESET_TEMPLATE)
            .await?;

        let email_reset_template = self
            .get_varying_secret(keys::email::EMAIL_RESET_TEMPLATE)
            .await?;

        let (api_key, sender_email) = match (api_key, sender_email) {
            (Some(api_key), Some(sender_email)) => (api_key, sender_email),
            _ => return Ok(None),
        };

        Ok(Some(EmailClientSettings {
            api_key,
            sender_email,
            signup_verify_template,
            password_reset_template,
            email_reset_template,
        }))
    }

    /// Load the valid google access token information
    pub async fn google_cloud_serivce_token(
        &self,
    ) -> anyhow::Result<Option<GoogleAccessTokenResponse>> {
        let response = match (self.token.clone(), self.expires_at.clone()) {
            (None, None) => None,
            (access_token, expires_at) => Some(GoogleAccessTokenResponse {
                access_token,
                expires_at,
            }),
        };

        Ok(response)
    }

    /// Load the google cloud storage Client settings
    pub async fn google_cloud_storage_settings(
        &self,
    ) -> anyhow::Result<Option<GoogleCloudStorageSettings>> {
        // buckets are the same as those for S3, but uses the GCS API instead of S3 interop
        let media_bucket = match self.remote_target.s3_bucket() {
            Some(bucket) => Some(bucket.to_string()),
            None => self.get_varying_secret(keys::s3::MEDIA_BUCKET).await?,
        };

        let processing_bucket = match self.remote_target.s3_processing_bucket() {
            Some(bucket) => Some(bucket.to_string()),
            None => self.get_varying_secret(keys::s3::PROCESSING_BUCKET).await?,
        };

        match (media_bucket, processing_bucket) {
            (Some(media_bucket), Some(processing_bucket)) => Ok(Some(GoogleCloudStorageSettings {
                media_bucket,
                processing_bucket,
            })),

            _ => return Ok(None),
        }
    }

    /// Load the Google Cloud EventArc settings
    pub async fn google_cloud_eventarc_settings(
        &self,
    ) -> anyhow::Result<Option<GoogleCloudEventArcSettings>> {
        let project_id = Some(self.project_id.clone());

        let storage_service_name = Some(EVENTARC_AUDITLOG_SERVICE_NAME.to_owned());

        let media_uploaded_topic = match self.remote_target.google_eventarc_media_uploaded_topic() {
            Some(endpoint) => Some(endpoint.to_string()),
            None => {
                self.get_varying_secret(keys::event_arc::MEDIA_UPLOADED_TOPIC)
                    .await?
            }
        };

        let media_processed_topic = match self.remote_target.google_eventarc_media_processed_topic()
        {
            Some(endpoint) => Some(endpoint.to_string()),
            None => {
                self.get_varying_secret(keys::event_arc::MEDIA_PROCESSED_TOPIC)
                    .await?
            }
        };

        match (
            project_id,
            storage_service_name,
            media_uploaded_topic,
            media_processed_topic,
        ) {
            (
                Some(project_id),
                Some(storage_service_name),
                Some(media_uploaded_topic),
                Some(media_processed_topic),
            ) => Ok(Some(GoogleCloudEventArcSettings {
                media_uploaded_topic,
                media_processed_topic,
                storage_service_name,
                project_id,
            })),
            _ => Ok(None),
        }
    }

    /// Load the settings for firebase
    pub async fn firebase_settings(&self) -> anyhow::Result<Option<FirebaseSettings>> {
        let project_id = Some(self.project_id.clone());

        match project_id {
            Some(project_id) => Ok(Some(FirebaseSettings { project_id })),
            _ => Ok(None),
        }
    }

    /// Load the audience targets for
    pub async fn jwk_audience_settings(
        &self,
        runtime_settings: &RuntimeSettings,
    ) -> anyhow::Result<JwkAudiences> {
        Ok(JwkAudiences {
            oauth_client: runtime_settings
                .google_oauth
                .as_ref()
                .map_or_else(String::new, |it| it.client.clone()),
            api: self.remote_target.api_assigned_url(),
            media_watch: self
                .remote_target
                .media_watch_assigned_url()
                .map_or_else(String::new, |it| it.to_owned()),
        })
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

        RuntimeSettings::with_env(
            self.remote_target,
            bing_search_key,
            google_oauth,
            token_secret,
            login_token_valid_duration,
        )
    }
}

fn get_epoch() -> std::time::Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
