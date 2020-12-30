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
    pub const APPLICATION_ID: &str = "ALGOLIA_PROJECT_ID";

    pub const IMAGE_INDEX: &str = "ALGOLIA_IMAGE_INDEX";

    pub const KEY: &str = "ALGOLIA_KEY";
    pub const DISABLE: &str = "ALGOLIA_LOCAL_DISABLE_CLIENT";
}

pub const JWT_SECRET: &str = "JWT_SECRET";

pub const SENTRY_DSN_API: &str = "SENTRY_DSN_API";
pub const SENTRY_DSN_PAGES: &str = "SENTRY_DSN_PAGES";

pub const BING_SEARCH_KEY: &str = "BING_SEARCH_KEY";
