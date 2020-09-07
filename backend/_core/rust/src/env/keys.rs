pub mod google {
    pub const DISABLE: &str = "DISABLE_GOOGLE_CLOUD";

    pub const PROJECT_ID: &str = "PROJECT_ID";
}

pub mod s3 {
    pub const ENDPOINT: &str = "S3_ENDPOINT";

    pub const BUCKET: &str = "S3_BUCKET";

    pub const ACCESS_KEY_OLD: &str = "S3_ACCESS_KEY_ID";
    pub const ACCESS_KEY_NEW: &str = "GOOGLE_S3_ACCESS_KEY";

    pub const SECRET_OLD: &str = "S3_SECRET_ACCESS_KEY";
    pub const SECRET_NEW: &str = "GOOGLE_S3_ACCESS_SECRET";

    pub const DISABLE: &str = "S3_LOCAL_DISABLE_CLIENT";
}

pub mod db {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const PASSWORD: &str = "DB_PASS";
    pub const INSTANCE_CONNECTION_NAME: &str = "INSTANCE_CONNECTION_NAME";
    pub const SOCKET_PATH: &str = "DB_SOCKET_PATH";
}

pub mod algolia {
    pub const APPLICATION_ID_OLD: &str = "ALGOLIA_APPLICATION_ID";
    pub const APPLICATION_ID_NEW: &str = "ALGOLIA_PROJECT_ID";

    pub const KEY: &str = "ALGOLIA_KEY";
    pub const DISABLE: &str = "ALGOLIA_LOCAL_DISABLE_CLIENT";
}

pub const JWT_SECRET: &str = "JWT_SECRET";
