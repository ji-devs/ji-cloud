//TODO - read from env to get local/dev port settings and minio bucket name
use time::Duration;

pub const STAGE_WIDTH: f64 = 1920.0;
pub const STAGE_HEIGHT: f64 = 1080.0;
pub const STAGE_PADDING_Y_PERC: f64 = 0.05; // in percentage, to offset the stage area a bit
pub const STAGE_PADDING_X_PERC: f64 = 0.05;
pub const STAGE_RATIO: f64 = STAGE_WIDTH / STAGE_HEIGHT;

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

pub const IMAGE_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 10; // 10 MB

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

    pub const fn algolia_image_index(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox => Some("image_sandbox"),
            Self::Release => Some("image_release"),
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

    pub const fn uploads_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:9000/test-bucket",
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
            Self::Local => "http://localhost:10001",
            Self::Sandbox => "https://sandbox.jicloud.org",
            Self::Release => "https://jicloud.org",
        }
    }
    pub const fn pages_url_iframe(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:4105",
            Self::Sandbox => "https://sandbox.jicloud.org",
            Self::Release => "https://jicloud.org",
        }
    }

    pub const fn frontend_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:4104",
            Self::Sandbox => "https://frontend.sandbox.jicloud.org",
            Self::Release => "https://frontend.jicloud.org",
        }
    }

    pub fn css_url(&self, minified: bool) -> String {
        if minified {
            format!("{}/_css/styles.min.css", self.frontend_url())
        } else {
            format!("{}/_css/styles.css", self.frontend_url())
        }
    }

    pub fn spa_url(&self, app: &str, path: &str) -> String {
        format!("{}/{}/{}", self.frontend_url(), app, path)
    }

    //route_path is the full path, i.e. what comes from Route::into on the frontend
    pub fn spa_iframe(&self, route_path: &str) -> String {
        format!("{}{}", self.pages_url_iframe(), route_path)
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

    pub fn google_maps_url(&self) -> &'static str {
        match self {
            // these are _apparently_ public?
            Self::Local
            | Self::Sandbox => "https://maps.googleapis.com/maps/api/js?key=AIzaSyCtU4taX_GG36bXfZr98HSwZTBNYo9HS1I&libraries=places",
            Self::Release => "https://maps.googleapis.com/maps/api/js?key=AIzaSyCU1HygSZgK4L3qPdRmrV-dTnS1GBBiqyE&libraries=places"
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Sandbox => "sandbox",
            Self::Release => "release",
        }
    }
}
