pub mod google {
    pub const DISABLE: &str = "DISABLE_GOOGLE_CLOUD";

    pub const PROJECT_ID: &str = "PROJECT_ID";
}

pub mod s3 {
    /// The s3 endpoint to connect to.
    /// Is optional. If missing, all s3 related services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const ENDPOINT: &str = "S3_ENDPOINT";

    /// The s3 bucket that should be used for media.
    /// Is optional. If missing, all s3 related services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const MEDIA_BUCKET: &str = "S3_BUCKET";

    /// The s3 bucket that should be used for media to be processed.
    /// Is optional. If missing, all s3 related services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.

    pub const PROCESSING_BUCKET: &str = "S3_PROCESSING_BUCKET";

    /// The s3 access key.
    /// Is optional. If missing, all s3 related services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const ACCESS_KEY: &str = "GOOGLE_S3_ACCESS_KEY";

    /// The s3 access key's secret.
    /// Is optional. If missing, all s3 related services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const SECRET: &str = "GOOGLE_S3_ACCESS_SECRET";

    /// Disable S3 locally (avoiding the warnings for missing secrets)
    /// if specified in a way that maps to `true` (currently "true", "1", "y"), all s3 related services will be disabled
    /// all related routes will return "501 - Not Implemented".
    pub const DISABLE: &str = "S3_LOCAL_DISABLE_CLIENT";
}

pub mod email {
    /// Disable email client locally (avoiding the warnings for missing secrets)
    /// If specified in a way that maps to `true` (currently "true", "1", "y"), all mailing related services,
    /// such as email verification, password resetting, and email changing(1) will be disabled.
    /// all related routes will return "501 - Not Implemented"
    /// (1): currently not implemented.
    pub const DISABLE: &str = "SENDGRID_DISABLE_LOCAL";

    /// Sendgrid / email client api key.
    // Is optional. If missing, all mailing services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const API_KEY: &str = "SENDGRID_API_KEY";

    /// Email client sender email address.
    /// Is optional. If missing, all mailing services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const SENDER_EMAIL: &str = "EMAIL_SENDER_ADDRESS";

    /// Email client template ID for verifying emails at signup.
    /// Is optional. If missing, email verification (at signup) will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const SIGNUP_VERIFY_TEMPLATE: &str = "EMAIL_SIGNUP_VERIFY_TEMPLATE";

    /// Email client template ID for resetting passwords.
    /// Is optional. If missing, password resetting will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const PASSWORD_RESET_TEMPLATE: &str = "EMAIL_PASSWORD_RESET_TEMPLATE";
}

#[cfg(feature = "db")]
pub mod db {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const PASSWORD: &str = "DB_PASS";
    pub const INSTANCE_CONNECTION_NAME: &str = "INSTANCE_CONNECTION_NAME";
    pub const SOCKET_PATH: &str = "DB_SOCKET_PATH";
}

pub mod algolia {
    /// The ID of the algolia application.
    /// Is optional. If missing, all algolia related services will be disabled,
    /// all related routes will return "501 - Not Implemented" and a warning will be emitted.
    pub const APPLICATION_ID: &str = "ALGOLIA_PROJECT_ID";

    /// The index to use for indexing and backend searches.
    /// Is optional. If missing, indexing will be disabled,
    /// search related routes will return a "501 - Not Implemented" and a warning will be emitted.
    pub const MEDIA_INDEX: &str = "ALGOLIA_MEDIA_INDEX";

    /// The index to use for indexing and backend searches.
    /// Is optional. If missing, *jig* indexing will be disabled,
    /// search related routes will return a "501 - Not Implemented" and a warning will be emitted.
    pub const JIG_INDEX: &str = "ALGOLIA_JIG_INDEX";

    /// The key the backend uses for managing- indexing- [`MEDIA_INDEX`] and [`JIG_INDEX`].
    /// Needs the `addObject`, `deleteObject`, `settings`, and `editSettings` ACLs, access to `MEDIA_INDEX`, and `JIG_INDEX`.
    /// Is optional. If missing, indexing will be disabled, and a warning will be logged.
    pub const MANAGEMENT_KEY: &str = "ALGOLIA_MANAGEMENT_KEY";

    /// The key that the backend uses for searching [`MEDIA_INDEX`] and [`JIG_INDEX`].
    /// Needs the `search` ACL with access to `MEDIA_INDEX` and `JIG_INDEX`.
    /// Is optional. If missing, searching will be disabled, attempting
    /// to use search related routes will return a "501 - Not Implemented" and a warning will be logged.
    pub const BACKEND_SEARCH_KEY: &str = "ALGOLIA_BACKEND_SEARCH_KEY";

    /// The key to use for the *frontend* for the algolia client.
    /// This key should be ratelimited, and restricted to a specific set of indecies:
    /// *possibly* `MEDIA_INDEX` and *definitely* any search suggestion indecies related to it.
    /// Is optional, if not present, routes related to creating search keys for the frontend will return "501 - Not Implemented" and a warning will be logged.
    pub const FRONTEND_SEARCH_KEY: &str = "ALGOLIA_FRONTEND_SEARCH_KEY";
}

/// Must be 32 bytes of hex
pub const TOKEN_SECRET: &str = "TOKEN_SECRET";

/// How long *login* tokens are valid for (measured in seconds).
/// This environment variable can only be set on `local`
/// This environment variable is optional, if missing it will use the server's compiled default (an indeterminate but reasonable amount of time)
pub const LOGIN_TOKEN_VALID_DURATION: &str = "LOGIN_TOKEN_VALID_DURATION";

pub const SENTRY_DSN_API: &str = "SENTRY_DSN_API";
pub const SENTRY_DSN_PAGES: &str = "SENTRY_DSN_PAGES";

pub const BING_SEARCH_KEY: &str = "BING_SEARCH_KEY";

/// ID of the google oauth client.
/// Is optional. If missing, all google-oauth related services will be disabled,
/// all related routes will return "501 - Not Implemented" and a warning will be emitted.
pub const GOOGLE_OAUTH_CLIENT: &str = "GOOGLE_OAUTH_CLIENT";

/// Secret for the google oauth client.
/// Is optional. If missing, all google-oauth related services will be disabled,
/// all related routes will return "501 - Not Implemented" and a warning will be emitted.
pub const GOOGLE_OAUTH_SECRET: &str = "GOOGLE_OAUTH_SECRET";
