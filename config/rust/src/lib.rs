use time::Duration;

pub const MEDIA_UI_PATH: &str = "ui";
pub const MAX_SIGNIN_COOKIE_DURATION: Duration = Duration::weeks(2); // 2 weeks
pub const JWK_ISSUER_URL: &str = "https://securetoken.google.com";
pub const JWK_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

/// `MAX_SIGNIN_COOKIE_DURATION` but as seconds,
/// as there's no way to get the string number of seconds from it `const`ly
#[deprecated = "use `MAX_SIGNIN_COOKIE_DURATION.whole_seconds()` instead"]
pub const MAX_SIGNIN_COOKIE: &str = "1209600";
pub const JSON_BODY_LIMIT: u64 = 1024 * 16; // 16
pub const COOKIE_DOMAIN: &str = "jicloud.org";
pub const CORS_ORIGINS: [&str; 2] = ["https://jicloud.org", "https://sandbox.jicloud.org"];
pub const DB_POOL_CONNECTIONS: u32 = 5;

pub const REMOTE_DB_USER: &str = "postgres";
pub const REMOTE_DB_NAME: &str = "jicloud";
pub const SQL_PROXY_PORT: u16 = 6432; //must match the port number in build-utils/package.json where cloud-sql-proxy is launched

pub const DB_INSTANCE_SANDBOX: &str =
    "ji-cloud-developer-sandbox:europe-west1:ji-cloud-003-sandbox";
pub const DB_INSTANCE_RELEASE: &str = "ji-cloud:europe-west1:ji-cloud-002";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}

impl RemoteTarget {
    pub const fn s3_endpoint(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox | Self::Release => Some("https://storage.googleapis.com"),
        }
    }

    pub const fn s3_bucket(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox => Some("ji-cloud-sandbox-uploads-origin-eu-001"),
            Self::Release => Some("ji-cloud-uploads-origin-eu-001"),
        }
    }

    pub const fn google_credentials_env_name(&self) -> &'static str {
        match self {
            Self::Local => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
            Self::Sandbox => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
            Self::Release => "GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE",
        }
    }

    pub const fn api_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8080",
            Self::Sandbox => "https://api.sandbox.jicloud.org",
            Self::Release => "https://api.jicloud.org",
        }
    }

    pub const fn upload_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:9000",
            Self::Sandbox => "https://uploads.sandbox.jicloud.org",
            Self::Release => "https://uploads.jicloud.org",
        }
    }

    pub const fn media_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:4102",
            Self::Sandbox | Self::Release => "https://media.jicloud.org",
        }
    }

    pub const fn pages_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8081",
            Self::Sandbox => "https://sandbox.jicloud.org",
            Self::Release => "https://jicloud.org",
        }
    }

    pub const fn frontend_url(&self) -> &'static str {
        match self {
            Self::Local | Self::Sandbox => "https://frontend.sandbox.jicloud.org",
            Self::Release => "https://frontend.jicloud.org",
        }
    }

    pub fn spa_url(&self, app: &str, path: &str) -> String {
        format!("{}/{}/{}", self.frontend_url(), app, path)
    }

    pub const fn host(&self) -> Option<&'static str> {
        None
    }

    pub fn media_ui_url(&self) -> String {
        format!("{}/ui", self.media_url())
    }

    pub fn replace_media_ui<S: AsRef<str>>(&self, s: S) -> String {
        s.as_ref()
            .replace("%MEDIA_UI%", &format!("{}/ui", self.media_url()))
    }
}
