pub mod google {
    pub const DISABLE: &str = "DISABLE_GOOGLE_CLOUD";

    pub const PROJECT_ID: &str = "PROJECT_ID";
}

pub mod s3 {
    pub const ENDPOINT: &str = "S3_ENDPOINT";

    pub const BUCKET: &str = "S3_BUCKET";

    pub const ACCESS_KEY: &str = "GOOGLE_S3_ACCESS_KEY";

    pub const SECRET: &str = "GOOGLE_S3_ACCESS_SECRET";

    pub const DISABLE: &str = "S3_LOCAL_DISABLE_CLIENT";
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

    /// The key the backend uses for managing- indexing- `MEDIA_INDEX`.
    /// Needs the `addObject`, `deleteObject`, `settings`, and `editSettings` ACLs and access to `MEDIA_INDEX`.
    /// Is optional. If missing, indexing will be disabled, and a warning will be logged.
    pub const MANAGEMENT_KEY: &str = "ALGOLIA_MANAGEMENT_KEY";

    /// The key that the backend uses for searching `MEDIA_INDEX`.
    /// Needs the `search` ACL with access to `MEDIA_INDEX`.
    /// Is optional. If missing, searching will be disabled, attempting
    /// to use search related routes will return a "501 - Not Implemented" and a warning will be logged.
    pub const BACKEND_SEARCH_KEY: &str = "ALGOLIA_BACKEND_SEARCH_KEY";

    /// The key to use for the *frontend* for the algolia client.
    /// This key should be ratelimited, and restricted to a specific set of indecies:
    /// *possibly* `MEDIA_INDEX` and *definitely* any search suggestion indecies related to it.
    /// Is optional, if not present, routes related to creating search keys for the frontend will return "501 - Not Implemented" and a warning will be logged.
    pub const FRONTEND_SEARCH_KEY: &str = "ALGOLIA_FRONTEND_SEARCH_KEY";
}

pub const JWT_SECRET: &str = "JWT_SECRET";

pub const SENTRY_DSN_API: &str = "SENTRY_DSN_API";
pub const SENTRY_DSN_PAGES: &str = "SENTRY_DSN_PAGES";

pub const BING_SEARCH_KEY: &str = "BING_SEARCH_KEY";
