use std::env::VarError;

pub const MEDIA_UI_PATH: &str = "ui";
pub const JWK_ISSUER_URL: &str = "https://accounts.google.com";
pub const JWK_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

pub const EVENTARC_AUDITLOG_SERVICE_NAME: &str = "cloudaudit.googleapis.com";

/// `MAX_SIGNIN_COOKIE_DURATION` but as seconds,
/// as there's no way to get the string number of seconds from it `const`ly
#[deprecated = "use `MAX_SIGNIN_COOKIE_DURATION.whole_seconds()` instead"]
pub const MAX_SIGNIN_COOKIE: &str = "1209600";
pub const JSON_BODY_LIMIT: u64 = 1024 * 16; // 16
pub const COOKIE_DOMAIN: &str = "jicloud.org";
pub const CORS_ORIGINS: &[&str] = &[
    "https://jicloud.org",
    "https://sandbox.jicloud.org",
    "https://api.jicloud.org",
    "https://api.sandbox.jicloud.org",
    "http://localhost:4104",
    "http://localhost:4105",
];
pub const DB_POOL_CONNECTIONS: u32 = 5;

pub const IMAGE_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 10; // 10 MB

// Animations are reasonably expected to be larger than normal images?
pub const ANIMATION_BODY_SIZE_LIMIT: usize = 1024 * 1024 * 15; // 15 MB

pub const REMOTE_DB_USER: &str = "postgres";
pub const REMOTE_DB_NAME: &str = "jicloud";
pub const SQL_PROXY_PORT: u16 = 6432; //must match the port number in build-utils/package.json where cloud-sql-proxy is launched

pub const DB_INSTANCE_SANDBOX: &str =
    "ji-cloud-developer-sandbox:europe-west1:ji-cloud-003-sandbox";
pub const DB_INSTANCE_RELEASE: &str = "ji-cloud:europe-west1:ji-cloud-002";

pub const MEDIA_UPLOAD_TIMEOUT_SECS: u64 = 300;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        #[wasm_bindgen(inline_js = "export function process_env_var(key) { const value = process.env[key]; return value == undefined ? '' : value; }")]
        extern "C" {
            #[wasm_bindgen(catch)]
            fn process_env_var(key:&str) -> Result<String, JsValue>;
        }

        pub fn env_var(key: &str) -> Result<String, VarError> {
            process_env_var(key)
                .map_err(|_| {
                    VarError::NotPresent
                })
                .and_then(|var| if var.is_empty() { Err(VarError::NotPresent) } else { Ok(var) })
        }
    } else {
        pub fn env_var(key: &str) -> Result<String, VarError> {
            std::env::var(key)
        }
    }
}

impl RemoteTarget {
    pub const fn s3_endpoint(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox | Self::Release => Some("https://storage.googleapis.com"),
        }
    }

    pub const fn s3_processing_bucket(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox => Some("ji-cloud-sandbox-processing-eu-001"),
            Self::Release => Some("ji-cloud-processing-eu-001"),
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

    pub const fn google_eventarc_media_uploaded_topic(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox => Some("eventarc-global-trigger-media-uploaded-sandbox-959"),
            Self::Release => Some("eventarc-global-trigger-media-uploaded-197"),
        }
    }

    pub const fn google_eventarc_media_processed_topic(&self) -> Option<&'static str> {
        match self {
            Self::Local => None,
            Self::Sandbox => Some("media-processed-sandbox"),
            Self::Release => Some("media-processed"),
        }
    }

    pub fn api_url(&self) -> String {
        match self {
            Self::Local => env_var("LOCAL_API_URL").unwrap_or("http://localhost:8080".to_string()),
            Self::Sandbox => "https://api.sandbox.jicloud.org".to_string(),
            Self::Release => "https://api.jicloud.org".to_string(),
        }
    }

    pub fn uploads_url(&self) -> String {
        match self {
            Self::Local => env_var("LOCAL_UPLOADS_URL")
                .unwrap_or("http://localhost:9000/test-bucket".to_string()),
            Self::Sandbox => "https://uploads.sandbox.jicloud.org".to_string(),
            Self::Release => "https://uploads.jicloud.org".to_string(),
        }
    }

    pub fn media_url(&self) -> String {
        match self {
            Self::Local => {
                env_var("LOCAL_MEDIA_URL").unwrap_or("http://localhost:4102".to_string())
            }
            Self::Sandbox | Self::Release => "https://media.jicloud.org".to_string(),
        }
    }

    pub fn pages_url(&self) -> String {
        match self {
            Self::Local => {
                env_var("LOCAL_PAGES_URL").unwrap_or("http://localhost:4104".to_string())
            }
            Self::Sandbox => "https://sandbox.jicloud.org".to_string(),
            Self::Release => "https://jicloud.org".to_string(),
        }
    }
    pub fn pages_url_iframe(&self) -> String {
        match self {
            Self::Local => {
                env_var("LOCAL_PAGES_URL_IFRAME").unwrap_or("http://localhost:4105".to_string())
            }
            Self::Sandbox => "https://sandbox.jicloud.org".to_string(),
            Self::Release => "https://jicloud.org".to_string(),
        }
    }

    pub fn frontend_url(&self) -> String {
        match self {
            Self::Local => {
                env_var("LOCAL_FRONTEND_URL").unwrap_or("http://localhost:4104".to_string())
            }
            Self::Sandbox => "https://frontend.sandbox.jicloud.org".to_string(),
            Self::Release => "https://frontend.jicloud.org".to_string(),
        }
    }

    pub fn css_url(&self, _minified: bool) -> String {
        format!("{}/css/head.css", self.media_ui_url())
        /*
        if minified {
            format!("{}/_css/styles.min.css", self.frontend_url())
        } else {
            format!("{}/_css/styles.css", self.frontend_url())
        }
        */
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
